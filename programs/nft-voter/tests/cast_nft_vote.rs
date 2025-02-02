
use crate::program_test::nft_voter_test::ConfigureCollectionArgs;
use gpl_nft_voter::error::NftVoterError;
use gpl_nft_voter::state::*;
use program_test::token_metadata_test::CreateNftArgs;
use program_test::{nft_voter_test::*, tools::assert_nft_voter_err};

use solana_program_test::*;
use spl_governance::state::vote_record::{Vote, VoteChoice};

mod program_test;

#[tokio::test]
async fn test_cast_nft_vote_with_phase_spl_deny_weight_should_be_0() -> Result<(), BanksClientError>
{
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, Some(vec!["Reject".to_string()]))
        .await?;

    let transaction_cookie = nft_voter_test
        .governance
        .with_add_tx(&proposal_cookie)
        .await?;

    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    nft_voter_test.bench.advance_clock().await;
    let clock = nft_voter_test.bench.get_clock().await;

    // Act
    let nft_vote_record_cookies = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            Some(transaction_cookie),
            Some(CastNftVoteArgs {
                cast_spl_gov_vote: true,
                vote_type: Vote::Deny,
            }),
        )
        .await?;

    // Assert
    let nft_vote_record = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[0].address)
        .await;

    assert_eq!(nft_vote_record_cookies[0].account, nft_vote_record);

    let voter_weight_record = nft_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 0);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    nft_voter_test
        .governance
        .with_add_tx(&proposal_cookie)
        .await?;
    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    nft_voter_test.bench.advance_clock().await;
    let clock = nft_voter_test.bench.get_clock().await;

    // Act
    let nft_vote_record_cookies = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            None,
            None,
        )
        .await?;

    // Assert
    let nft_vote_record = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[0].address)
        .await;

    assert_eq!(nft_vote_record_cookies[0].account, nft_vote_record);

    let voter_weight_record = nft_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 10);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_multiple_nfts_with_phase_deny_should_be_0(
) -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, Some(vec!["Reject".to_string()]))
        .await?;

    let transaction_cookie = nft_voter_test
        .governance
        .with_add_tx(&proposal_cookie)
        .await?;

    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    let nft_cookie2 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    nft_voter_test.bench.advance_clock().await;
    let clock = nft_voter_test.bench.get_clock().await;

    // Act
    let nft_vote_record_cookies = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1, &nft_cookie2],
            Some(transaction_cookie),
            Some(CastNftVoteArgs {
                cast_spl_gov_vote: true,
                vote_type: Vote::Deny,
            }),
        )
        .await?;

    // Assert
    let nft_vote_record1 = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[0].address)
        .await;

    assert_eq!(nft_vote_record_cookies[0].account, nft_vote_record1);

    let nft_vote_record2 = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[1].address)
        .await;

    assert_eq!(nft_vote_record_cookies[1].account, nft_vote_record2);

    let voter_weight_record = nft_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 0);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_multiple_nfts() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    let nft_cookie2 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    nft_voter_test.bench.advance_clock().await;
    let clock = nft_voter_test.bench.get_clock().await;

    // Act
    let nft_vote_record_cookies = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1, &nft_cookie2],
            None,
            None,
        )
        .await?;

    // Assert
    let nft_vote_record1 = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[0].address)
        .await;

    assert_eq!(nft_vote_record_cookies[0].account, nft_vote_record1);

    let nft_vote_record2 = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[1].address)
        .await;

    assert_eq!(nft_vote_record_cookies[1].account, nft_vote_record2);

    let voter_weight_record = nft_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 20);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}
#[tokio::test]
async fn test_cast_nft_vote_with_phase_spl_deny_weight_without_prop_tx_should_fail(
) -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, Some(vec!["Reject".to_string()]))
        .await?;

    nft_voter_test
        .governance
        .with_add_tx(&proposal_cookie)
        .await?;
    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    nft_voter_test.bench.advance_clock().await;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            None,
            Some(CastNftVoteArgs {
                cast_spl_gov_vote: true,
                vote_type: Vote::Deny,
            }),
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(
        err,
        NftVoterError::MustIncludeProposalTransactionForPhaseVotes,
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_nft_already_voted_error() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            None,
            None,
        )
        .await?;

    nft_voter_test.bench.advance_clock().await;

    // Act

    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            None,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::NftAlreadyVoted);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_invalid_voter_error() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    let voter_cookie2 = nft_voter_test.bench.with_wallet().await;

    // Act

    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie2,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            None,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::InvalidTokenOwnerForVoterWeightRecord);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_unverified_collection_error() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    // Create NFT without verified collection
    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(
            &nft_collection_cookie,
            &voter_cookie,
            Some(CreateNftArgs {
                verify_collection: false,
                ..Default::default()
            }),
        )
        .await?;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            None,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::CollectionMustBeVerified);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_invalid_owner_error() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let voter_cookie2 = nft_voter_test.bench.with_wallet().await;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie2, None)
        .await?;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie],
            None,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::VoterDoesNotOwnNft);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_invalid_collection_error() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let nft_collection_cookie2 = nft_voter_test.token_metadata.with_nft_collection().await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie2, &voter_cookie, None)
        .await?;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie],
            None,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::CollectionNotFound);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_invalid_metadata_error() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let mut nft1_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(
            &nft_collection_cookie,
            &voter_cookie,
            Some(CreateNftArgs {
                verify_collection: false,
                ..Default::default()
            }),
        )
        .await?;

    let nft2_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    // Try to use verified NFT Metadata
    nft1_cookie.metadata = nft2_cookie.metadata;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft1_cookie],
            None,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::TokenMetadataDoesNotMatch);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_same_nft_error() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    nft_voter_test
        .governance
        .with_add_tx(&proposal_cookie)
        .await?;
    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie, &nft_cookie],
            None,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_nft_voter_err(err, NftVoterError::DuplicatedNftDetected);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_no_nft_error() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(
            &nft_collection_cookie,
            &voter_cookie,
            Some(CreateNftArgs {
                amount: 0,
                ..Default::default()
            }),
        )
        .await?;

    // Act
    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            None,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::InvalidNftAmount);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_max_5_nfts_with_deny_phase_vote() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    let transaction_cookie = nft_voter_test
        .governance
        .with_add_tx(&proposal_cookie)
        .await?;

    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let mut nft_cookies = vec![];

    for _ in 0..5 {
        nft_voter_test.bench.advance_clock().await;
        let nft_cookie = nft_voter_test
            .token_metadata
            .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
            .await?;

        nft_cookies.push(nft_cookie)
    }

    nft_voter_test.bench.advance_clock().await;
    let clock = nft_voter_test.bench.get_clock().await;

    // Act
    let nft_vote_record_cookies = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &nft_cookies.iter().collect::<Vec<_>>(),
            Some(transaction_cookie),
            Some(CastNftVoteArgs {
                cast_spl_gov_vote: true,
                vote_type: Vote::Deny,
            }),
        )
        .await?;

    // Assert
    let nft_vote_record1 = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[0].address)
        .await;

    assert_eq!(nft_vote_record_cookies[0].account, nft_vote_record1);

    let nft_vote_record2 = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[1].address)
        .await;

    assert_eq!(nft_vote_record_cookies[1].account, nft_vote_record2);

    let voter_weight_record = nft_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 50);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_with_max_5_nfts() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    nft_voter_test
        .governance
        .with_add_tx(&proposal_cookie)
        .await?;
    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let mut nft_cookies = vec![];

    for _ in 0..5 {
        nft_voter_test.bench.advance_clock().await;
        let nft_cookie = nft_voter_test
            .token_metadata
            .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
            .await?;

        nft_cookies.push(nft_cookie)
    }

    nft_voter_test.bench.advance_clock().await;
    let clock = nft_voter_test.bench.get_clock().await;

    // Act
    let nft_vote_record_cookies = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &nft_cookies.iter().collect::<Vec<_>>(),
            None,
            None,
        )
        .await?;

    // Assert
    let nft_vote_record1 = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[0].address)
        .await;

    assert_eq!(nft_vote_record_cookies[0].account, nft_vote_record1);

    let nft_vote_record2 = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[1].address)
        .await;

    assert_eq!(nft_vote_record_cookies[1].account, nft_vote_record2);

    let voter_weight_record = nft_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 50);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_using_multiple_instructions() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    nft_voter_test
        .governance
        .with_add_tx(&proposal_cookie)
        .await?;
    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    nft_voter_test.bench.advance_clock().await;
    let clock = nft_voter_test.bench.get_clock().await;

    let args = CastNftVoteArgs {
        cast_spl_gov_vote: false,
        vote_type: Vote::Approve(vec![VoteChoice {
            rank: 0,
            weight_percentage: 100,
        }]),
    };

    nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            None,
            Some(args),
        )
        .await?;

    let nft_cookie2 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    // Act

    nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie2],
            None,
            None,
        )
        .await?;

    // Assert

    let voter_weight_record = nft_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 20);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_using_multiple_instructions_with_nft_already_voted_error(
) -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    let transaction_cookie = nft_voter_test
        .governance
        .with_add_tx(&proposal_cookie)
        .await?;
    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    let args = CastNftVoteArgs {
        cast_spl_gov_vote: false,
        vote_type: Vote::Approve(vec![VoteChoice {
            rank: 0,
            weight_percentage: 100,
        }]),
    };

    nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            None,
            Some(args),
        )
        .await?;

    // Act

    let err = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            Some(transaction_cookie),
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::NftAlreadyVoted);

    Ok(())
}

#[tokio::test]
async fn test_cast_nft_vote_using_multiple_instructions_with_attempted_sandwiched_relinquish(
) -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie, None)
        .await?;

    nft_voter_test
        .governance
        .with_add_tx(&proposal_cookie)
        .await?;
    nft_voter_test
        .governance
        .with_sign_off_proposal(&proposal_cookie, &realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    let args = CastNftVoteArgs {
        cast_spl_gov_vote: false,
        vote_type: Vote::Approve(vec![VoteChoice {
            rank: 0,
            weight_percentage: 100,
        }]),
    };

    // Cast vote with NFT
    let nft_vote_record_cookies = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            None,
            Some(args),
        )
        .await?;

    nft_voter_test.bench.advance_clock().await;

    // Try relinquish NftVoteRecords to accumulate vote
    nft_voter_test
        .relinquish_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &nft_vote_record_cookies,
        )
        .await?;

    // Act

    nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
            None,
            None,
        )
        .await?;

    // Assert

    let voter_weight_record = nft_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 10);

    Ok(())
}
