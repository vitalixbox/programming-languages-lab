use a02_event_stream::{AccountEvent, AccountSummary, reduce_events};

#[test]
fn reduces_a_mixed_stream_for_the_target_account() {
    let events = vec![
        AccountEvent::Deposit {
            account_id: 42,
            amount: 600,
        },
        AccountEvent::Withdrawal {
            account_id: 42,
            amount: 125,
            approved: true,
        },
        AccountEvent::Withdrawal {
            account_id: 42,
            amount: 2_000,
            approved: false,
        },
        AccountEvent::AuditNote("manual review requested".to_owned()),
        AccountEvent::Close { account_id: 42 },
    ];

    let summary = reduce_events(events, 42);

    assert_eq!(
        summary,
        AccountSummary {
            total_deposited: 600,
            total_withdrawn: 125,
            declined_withdrawals: 1,
            large_withdrawal_seen: false,
            latest_freeze_reason: None,
            is_closed: true,
        }
    );
}

#[test]
fn ignores_events_for_other_accounts_and_audit_notes() {
    let events = vec![
        AccountEvent::Deposit {
            account_id: 7,
            amount: 900,
        },
        AccountEvent::Withdrawal {
            account_id: 7,
            amount: 5_000,
            approved: true,
        },
        AccountEvent::Withdrawal {
            account_id: 7,
            amount: 500,
            approved: false,
        },
        AccountEvent::Freeze {
            account_id: 7,
            reason: "fraud review".to_owned(),
        },
        AccountEvent::Close { account_id: 7 },
        AccountEvent::AuditNote("system maintenance".to_owned()),
    ];

    assert_eq!(reduce_events(events, 42), AccountSummary::default());
}

#[test]
fn keeps_the_latest_freeze_reason() {
    let events = vec![
        AccountEvent::Freeze {
            account_id: 42,
            reason: "unusual activity".to_owned(),
        },
        AccountEvent::Freeze {
            account_id: 42,
            reason: "compliance review".to_owned(),
        },
    ];

    let summary = reduce_events(events, 42);

    assert_eq!(
        summary.latest_freeze_reason.as_deref(),
        Some("compliance review")
    );
}

#[test]
fn an_approved_withdrawal_at_the_threshold_is_large() {
    let events = vec![AccountEvent::Withdrawal {
        account_id: 42,
        amount: 1_000,
        approved: true,
    }];

    let summary = reduce_events(events, 42);

    assert_eq!(summary.total_withdrawn, 1_000);
    assert!(summary.large_withdrawal_seen);
    assert_eq!(summary.declined_withdrawals, 0);
}

#[test]
fn a_declined_large_withdrawal_only_increments_the_declined_count() {
    let events = vec![AccountEvent::Withdrawal {
        account_id: 42,
        amount: 10_000,
        approved: false,
    }];

    let summary = reduce_events(events, 42);

    assert_eq!(summary.total_withdrawn, 0);
    assert!(!summary.large_withdrawal_seen);
    assert_eq!(summary.declined_withdrawals, 1);
}

#[test]
fn an_empty_stream_returns_the_default_summary() {
    assert_eq!(reduce_events(Vec::new(), 42), AccountSummary::default());
}
