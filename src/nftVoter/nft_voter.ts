export type NftVoter = {
  "version": "0.1.1",
  "name": "nft_voter",
  "instructions": [
    {
      "name": "createRegistrar",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The NFT voting Registrar",
            "There can only be a single registrar per governance Realm and governing mint of the Realm"
          ]
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The program id of the spl-governance program the realm belongs to"
          ]
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "An spl-governance Realm",
            "",
            "Realm is validated in the instruction:",
            "- Realm is owned by the governance_program_id",
            "- governing_token_mint must be the community or council mint",
            "- realm_authority is realm.authority"
          ]
        },
        {
          "name": "governingTokenMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Either the realm community mint or the council mint.",
            "It must match Realm.community_mint or Realm.config.council_mint",
            "",
            "Note: Once the NFT plugin is enabled the governing_token_mint is used only as identity",
            "for the voting population and the tokens of that are no longer used"
          ]
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "realm_authority must sign and match Realm.authority"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "maxCollections",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createVoterWeightRecord",
      "accounts": [
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The program id of the spl-governance program the realm belongs to"
          ]
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmGoverningTokenMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Either the realm community mint or the council mint."
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "governingTokenOwner",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "createMaxVoterWeightRecord",
      "accounts": [
        {
          "name": "maxVoterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The program id of the spl-governance program the realm belongs to"
          ]
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmGoverningTokenMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Either the realm community mint or the council mint."
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateVoterWeightRecord",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The NFT voting Registrar"
          ]
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "voterWeightAction",
          "type": {
            "defined": "VoterWeightAction"
          }
        }
      ]
    },
    {
      "name": "relinquishNftVote",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The NFT voting Registrar"
          ]
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governance",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Governance account the Proposal is for"
          ]
        },
        {
          "name": "proposal",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "governingTokenOwner",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "The token owner who cast the original vote"
          ]
        },
        {
          "name": "voteRecord",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The account is used to validate that it doesn't exist and if it doesn't then Anchor owner check throws error",
            "The check is disabled here and performed inside the instruction",
            "#[account(owner = registrar.governance_program_id)]"
          ]
        },
        {
          "name": "beneficiary",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "configureCollection",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Registrar for which we configure this Collection"
          ]
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "Authority of the Realm must sign and match Realm.authority"
          ]
        },
        {
          "name": "collection",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "maxVoterWeightRecord",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "weight",
          "type": "u64"
        },
        {
          "name": "size",
          "type": "u64"
        }
      ]
    },
    {
      "name": "castNftVote",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The NFT voting registrar"
          ]
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governingTokenOwner",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "The token owner who casts the vote"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The account which pays for the transaction"
          ]
        },
        {
          "name": "proposal",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "instructionSysvarAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "governanceProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "nftVoteRecord",
      "docs": [
        "NftVoteRecord exported to IDL without account_discriminator",
        "TODO: Once we can support these accounts in Anchor via remaining_accounts then it should be possible to remove it"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposal",
            "docs": [
              "Proposal which was voted on"
            ],
            "type": "publicKey"
          },
          {
            "name": "nftMint",
            "docs": [
              "The mint of the NFT which was used for the vote"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenOwner",
            "docs": [
              "The voter who casted this vote",
              "It's a Realm member pubkey corresponding to TokenOwnerRecord.governing_token_owner"
            ],
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "maxVoterWeightRecord",
      "docs": [
        "MaxVoterWeightRecord account as defined in spl-governance-addin-api",
        "It's redefined here without account_discriminator for Anchor to treat it as native account",
        "",
        "The account is used as an api interface to provide max voting power to the governance program from external addin contracts"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realm",
            "docs": [
              "The Realm the MaxVoterWeightRecord belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing Token Mint the MaxVoterWeightRecord is associated with",
              "Note: The addin can take deposits of any tokens and is not restricted to the community or council tokens only"
            ],
            "type": "publicKey"
          },
          {
            "name": "maxVoterWeight",
            "docs": [
              "Max voter weight",
              "The max voter weight provided by the addin for the given realm and governing_token_mint"
            ],
            "type": "u64"
          },
          {
            "name": "maxVoterWeightExpiry",
            "docs": [
              "The slot when the max voting weight expires",
              "It should be set to None if the weight never expires",
              "If the max vote weight decays with time, for example for time locked based weights, then the expiry must be set",
              "As a pattern Revise instruction to update the max weight should be invoked before governance instruction within the same transaction",
              "and the expiry set to the current slot to provide up to date weight"
            ],
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved space for future versions"
            ],
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    },
    {
      "name": "registrar",
      "docs": [
        "Registrar which stores NFT voting configuration for the given Realm"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "governanceProgramId",
            "docs": [
              "spl-governance program the Realm belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "realm",
            "docs": [
              "Realm of the Registrar"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing token mint the Registrar is for",
              "It can either be the Community or the Council mint of the Realm",
              "When the plugin is used the mint is only used as identity of the governing power (voting population)",
              "and the actual token of the mint is not used"
            ],
            "type": "publicKey"
          },
          {
            "name": "collectionConfigs",
            "docs": [
              "MPL Collection used for voting"
            ],
            "type": {
              "vec": {
                "defined": "CollectionConfig"
              }
            }
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved for future upgrades"
            ],
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    },
    {
      "name": "voterWeightRecord",
      "docs": [
        "VoterWeightRecord account as defined in spl-governance-addin-api",
        "It's redefined here without account_discriminator for Anchor to treat it as native account",
        "",
        "The account is used as an api interface to provide voting power to the governance program from external addin contracts"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realm",
            "docs": [
              "The Realm the VoterWeightRecord belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing Token Mint the VoterWeightRecord is associated with",
              "Note: The addin can take deposits of any tokens and is not restricted to the community or council tokens only"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenOwner",
            "docs": [
              "The owner of the governing token and voter",
              "This is the actual owner (voter) and corresponds to TokenOwnerRecord.governing_token_owner"
            ],
            "type": "publicKey"
          },
          {
            "name": "voterWeight",
            "docs": [
              "Voter's weight",
              "The weight of the voter provided by the addin for the given realm, governing_token_mint and governing_token_owner (voter)"
            ],
            "type": "u64"
          },
          {
            "name": "voterWeightExpiry",
            "docs": [
              "The slot when the voting weight expires",
              "It should be set to None if the weight never expires",
              "If the voter weight decays with time, for example for time locked based weights, then the expiry must be set",
              "As a common pattern Revise instruction to update the weight should be invoked before governance instruction within the same transaction",
              "and the expiry set to the current slot to provide up to date weight"
            ],
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "weightAction",
            "docs": [
              "The governance action the voter's weight pertains to",
              "It allows to provided voter's weight specific to the particular action the weight is evaluated for",
              "When the action is provided then the governance program asserts the executing action is the same as specified by the addin"
            ],
            "type": {
              "option": {
                "defined": "VoterWeightAction"
              }
            }
          },
          {
            "name": "weightActionTarget",
            "docs": [
              "The target the voter's weight  action pertains to",
              "It allows to provided voter's weight specific to the target the weight is evaluated for",
              "For example when addin supplies weight to vote on a particular proposal then it must specify the proposal as the action target",
              "When the target is provided then the governance program asserts the target is the same as specified by the addin"
            ],
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved space for future versions"
            ],
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CollectionConfig",
      "docs": [
        "Configuration of an NFT collection used for governance power"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "collection",
            "docs": [
              "The NFT collection used for governance"
            ],
            "type": "publicKey"
          },
          {
            "name": "size",
            "docs": [
              "The size of the NFT collection used to calculate max voter weight",
              "Note: At the moment the size is not captured on Metaplex accounts",
              "and it has to be manually updated on the Registrar"
            ],
            "type": "u64"
          },
          {
            "name": "weight",
            "docs": [
              "Governance power weight of the collection",
              "Each NFT in the collection has governance power = 1 * weight",
              "Note: The weight is scaled accordingly to the governing_token_mint decimals",
              "Ex: if the the mint has 2 decimal places then weight of 1 should be stored as 100"
            ],
            "type": "u64"
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved for future upgrades"
            ],
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    },
    {
      "name": "VoterWeightAction",
      "docs": [
        "VoterWeightAction enum as defined in spl-governance-addin-api",
        "It's redefined here for Anchor to export it to IDL"
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "CastVote"
          },
          {
            "name": "CommentProposal"
          },
          {
            "name": "CreateGovernance"
          },
          {
            "name": "CreateProposal"
          },
          {
            "name": "SignOffProposal"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidRealmAuthority",
      "msg": "Invalid Realm Authority"
    },
    {
      "code": 6001,
      "name": "InvalidRealmForRegistrar",
      "msg": "Invalid Realm for Registrar"
    },
    {
      "code": 6002,
      "name": "InvalidCollectionSize",
      "msg": "Invalid Collection Size"
    },
    {
      "code": 6003,
      "name": "InvalidMaxVoterWeightRecordRealm",
      "msg": "Invalid MaxVoterWeightRecord Realm"
    },
    {
      "code": 6004,
      "name": "InvalidMaxVoterWeightRecordMint",
      "msg": "Invalid MaxVoterWeightRecord Mint"
    },
    {
      "code": 6005,
      "name": "CastVoteIsNotAllowed",
      "msg": "CastVote Is Not Allowed"
    },
    {
      "code": 6006,
      "name": "InvalidVoterWeightRecordRealm",
      "msg": "Invalid VoterWeightRecord Realm"
    },
    {
      "code": 6007,
      "name": "InvalidVoterWeightRecordMint",
      "msg": "Invalid VoterWeightRecord Mint"
    },
    {
      "code": 6008,
      "name": "InvalidTokenOwnerForVoterWeightRecord",
      "msg": "Invalid TokenOwner for VoterWeightRecord"
    },
    {
      "code": 6009,
      "name": "CollectionMustBeVerified",
      "msg": "Collection must be verified"
    },
    {
      "code": 6010,
      "name": "VoterDoesNotOwnNft",
      "msg": "Voter does not own NFT"
    },
    {
      "code": 6011,
      "name": "CollectionNotFound",
      "msg": "Collection not found"
    },
    {
      "code": 6012,
      "name": "MissingMetadataCollection",
      "msg": "Missing Metadata collection"
    },
    {
      "code": 6013,
      "name": "TokenMetadataDoesNotMatch",
      "msg": "Token Metadata doesn't match"
    },
    {
      "code": 6014,
      "name": "InvalidAccountOwner",
      "msg": "Invalid account owner"
    },
    {
      "code": 6015,
      "name": "InvalidTokenMetadataAccount",
      "msg": "Invalid token metadata account"
    },
    {
      "code": 6016,
      "name": "DuplicatedNftDetected",
      "msg": "Duplicated NFT detected"
    },
    {
      "code": 6017,
      "name": "InvalidNftAmount",
      "msg": "Invalid NFT amount"
    },
    {
      "code": 6018,
      "name": "NftAlreadyVoted",
      "msg": "NFT already voted"
    },
    {
      "code": 6019,
      "name": "InvalidProposalForNftVoteRecord",
      "msg": "Invalid Proposal for NftVoteRecord"
    },
    {
      "code": 6020,
      "name": "InvalidTokenOwnerForNftVoteRecord",
      "msg": "Invalid TokenOwner for NftVoteRecord"
    },
    {
      "code": 6021,
      "name": "VoteRecordMustBeWithdrawn",
      "msg": "VoteRecord must be withdrawn"
    },
    {
      "code": 6022,
      "name": "InvalidVoteRecordForNftVoteRecord",
      "msg": "Invalid VoteRecord for NftVoteRecord"
    },
    {
      "code": 6023,
      "name": "VoterWeightRecordMustBeExpired",
      "msg": "VoterWeightRecord must be expired"
    },
    {
      "code": 6024,
      "name": "CannotConfigureCollectionWithVotingProposals",
      "msg": "Cannot configure collection with voting proposals"
    },
    {
      "code": 6025,
      "name": "MustIncludeProposalTransactionForPhaseVotes",
      "msg": "Must include proposals transaction since the proposal ressembles a phase vote"
    }
  ]
};

export const IDL: NftVoter = {
  "version": "0.1.1",
  "name": "nft_voter",
  "instructions": [
    {
      "name": "createRegistrar",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The NFT voting Registrar",
            "There can only be a single registrar per governance Realm and governing mint of the Realm"
          ]
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The program id of the spl-governance program the realm belongs to"
          ]
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "An spl-governance Realm",
            "",
            "Realm is validated in the instruction:",
            "- Realm is owned by the governance_program_id",
            "- governing_token_mint must be the community or council mint",
            "- realm_authority is realm.authority"
          ]
        },
        {
          "name": "governingTokenMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Either the realm community mint or the council mint.",
            "It must match Realm.community_mint or Realm.config.council_mint",
            "",
            "Note: Once the NFT plugin is enabled the governing_token_mint is used only as identity",
            "for the voting population and the tokens of that are no longer used"
          ]
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "realm_authority must sign and match Realm.authority"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "maxCollections",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createVoterWeightRecord",
      "accounts": [
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The program id of the spl-governance program the realm belongs to"
          ]
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmGoverningTokenMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Either the realm community mint or the council mint."
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "governingTokenOwner",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "createMaxVoterWeightRecord",
      "accounts": [
        {
          "name": "maxVoterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governanceProgramId",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The program id of the spl-governance program the realm belongs to"
          ]
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmGoverningTokenMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Either the realm community mint or the council mint."
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updateVoterWeightRecord",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The NFT voting Registrar"
          ]
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "voterWeightAction",
          "type": {
            "defined": "VoterWeightAction"
          }
        }
      ]
    },
    {
      "name": "relinquishNftVote",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The NFT voting Registrar"
          ]
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governance",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Governance account the Proposal is for"
          ]
        },
        {
          "name": "proposal",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "governingTokenOwner",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "The token owner who cast the original vote"
          ]
        },
        {
          "name": "voteRecord",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The account is used to validate that it doesn't exist and if it doesn't then Anchor owner check throws error",
            "The check is disabled here and performed inside the instruction",
            "#[account(owner = registrar.governance_program_id)]"
          ]
        },
        {
          "name": "beneficiary",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "configureCollection",
      "accounts": [
        {
          "name": "registrar",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Registrar for which we configure this Collection"
          ]
        },
        {
          "name": "realm",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "realmAuthority",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "Authority of the Realm must sign and match Realm.authority"
          ]
        },
        {
          "name": "collection",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "maxVoterWeightRecord",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "weight",
          "type": "u64"
        },
        {
          "name": "size",
          "type": "u64"
        }
      ]
    },
    {
      "name": "castNftVote",
      "accounts": [
        {
          "name": "registrar",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The NFT voting registrar"
          ]
        },
        {
          "name": "voterWeightRecord",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "governingTokenOwner",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "The token owner who casts the vote"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The account which pays for the transaction"
          ]
        },
        {
          "name": "proposal",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "instructionSysvarAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "governanceProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "nftVoteRecord",
      "docs": [
        "NftVoteRecord exported to IDL without account_discriminator",
        "TODO: Once we can support these accounts in Anchor via remaining_accounts then it should be possible to remove it"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposal",
            "docs": [
              "Proposal which was voted on"
            ],
            "type": "publicKey"
          },
          {
            "name": "nftMint",
            "docs": [
              "The mint of the NFT which was used for the vote"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenOwner",
            "docs": [
              "The voter who casted this vote",
              "It's a Realm member pubkey corresponding to TokenOwnerRecord.governing_token_owner"
            ],
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "maxVoterWeightRecord",
      "docs": [
        "MaxVoterWeightRecord account as defined in spl-governance-addin-api",
        "It's redefined here without account_discriminator for Anchor to treat it as native account",
        "",
        "The account is used as an api interface to provide max voting power to the governance program from external addin contracts"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realm",
            "docs": [
              "The Realm the MaxVoterWeightRecord belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing Token Mint the MaxVoterWeightRecord is associated with",
              "Note: The addin can take deposits of any tokens and is not restricted to the community or council tokens only"
            ],
            "type": "publicKey"
          },
          {
            "name": "maxVoterWeight",
            "docs": [
              "Max voter weight",
              "The max voter weight provided by the addin for the given realm and governing_token_mint"
            ],
            "type": "u64"
          },
          {
            "name": "maxVoterWeightExpiry",
            "docs": [
              "The slot when the max voting weight expires",
              "It should be set to None if the weight never expires",
              "If the max vote weight decays with time, for example for time locked based weights, then the expiry must be set",
              "As a pattern Revise instruction to update the max weight should be invoked before governance instruction within the same transaction",
              "and the expiry set to the current slot to provide up to date weight"
            ],
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved space for future versions"
            ],
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    },
    {
      "name": "registrar",
      "docs": [
        "Registrar which stores NFT voting configuration for the given Realm"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "governanceProgramId",
            "docs": [
              "spl-governance program the Realm belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "realm",
            "docs": [
              "Realm of the Registrar"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing token mint the Registrar is for",
              "It can either be the Community or the Council mint of the Realm",
              "When the plugin is used the mint is only used as identity of the governing power (voting population)",
              "and the actual token of the mint is not used"
            ],
            "type": "publicKey"
          },
          {
            "name": "collectionConfigs",
            "docs": [
              "MPL Collection used for voting"
            ],
            "type": {
              "vec": {
                "defined": "CollectionConfig"
              }
            }
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved for future upgrades"
            ],
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          }
        ]
      }
    },
    {
      "name": "voterWeightRecord",
      "docs": [
        "VoterWeightRecord account as defined in spl-governance-addin-api",
        "It's redefined here without account_discriminator for Anchor to treat it as native account",
        "",
        "The account is used as an api interface to provide voting power to the governance program from external addin contracts"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realm",
            "docs": [
              "The Realm the VoterWeightRecord belongs to"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenMint",
            "docs": [
              "Governing Token Mint the VoterWeightRecord is associated with",
              "Note: The addin can take deposits of any tokens and is not restricted to the community or council tokens only"
            ],
            "type": "publicKey"
          },
          {
            "name": "governingTokenOwner",
            "docs": [
              "The owner of the governing token and voter",
              "This is the actual owner (voter) and corresponds to TokenOwnerRecord.governing_token_owner"
            ],
            "type": "publicKey"
          },
          {
            "name": "voterWeight",
            "docs": [
              "Voter's weight",
              "The weight of the voter provided by the addin for the given realm, governing_token_mint and governing_token_owner (voter)"
            ],
            "type": "u64"
          },
          {
            "name": "voterWeightExpiry",
            "docs": [
              "The slot when the voting weight expires",
              "It should be set to None if the weight never expires",
              "If the voter weight decays with time, for example for time locked based weights, then the expiry must be set",
              "As a common pattern Revise instruction to update the weight should be invoked before governance instruction within the same transaction",
              "and the expiry set to the current slot to provide up to date weight"
            ],
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "weightAction",
            "docs": [
              "The governance action the voter's weight pertains to",
              "It allows to provided voter's weight specific to the particular action the weight is evaluated for",
              "When the action is provided then the governance program asserts the executing action is the same as specified by the addin"
            ],
            "type": {
              "option": {
                "defined": "VoterWeightAction"
              }
            }
          },
          {
            "name": "weightActionTarget",
            "docs": [
              "The target the voter's weight  action pertains to",
              "It allows to provided voter's weight specific to the target the weight is evaluated for",
              "For example when addin supplies weight to vote on a particular proposal then it must specify the proposal as the action target",
              "When the target is provided then the governance program asserts the target is the same as specified by the addin"
            ],
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved space for future versions"
            ],
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CollectionConfig",
      "docs": [
        "Configuration of an NFT collection used for governance power"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "collection",
            "docs": [
              "The NFT collection used for governance"
            ],
            "type": "publicKey"
          },
          {
            "name": "size",
            "docs": [
              "The size of the NFT collection used to calculate max voter weight",
              "Note: At the moment the size is not captured on Metaplex accounts",
              "and it has to be manually updated on the Registrar"
            ],
            "type": "u64"
          },
          {
            "name": "weight",
            "docs": [
              "Governance power weight of the collection",
              "Each NFT in the collection has governance power = 1 * weight",
              "Note: The weight is scaled accordingly to the governing_token_mint decimals",
              "Ex: if the the mint has 2 decimal places then weight of 1 should be stored as 100"
            ],
            "type": "u64"
          },
          {
            "name": "reserved",
            "docs": [
              "Reserved for future upgrades"
            ],
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    },
    {
      "name": "VoterWeightAction",
      "docs": [
        "VoterWeightAction enum as defined in spl-governance-addin-api",
        "It's redefined here for Anchor to export it to IDL"
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "CastVote"
          },
          {
            "name": "CommentProposal"
          },
          {
            "name": "CreateGovernance"
          },
          {
            "name": "CreateProposal"
          },
          {
            "name": "SignOffProposal"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidRealmAuthority",
      "msg": "Invalid Realm Authority"
    },
    {
      "code": 6001,
      "name": "InvalidRealmForRegistrar",
      "msg": "Invalid Realm for Registrar"
    },
    {
      "code": 6002,
      "name": "InvalidCollectionSize",
      "msg": "Invalid Collection Size"
    },
    {
      "code": 6003,
      "name": "InvalidMaxVoterWeightRecordRealm",
      "msg": "Invalid MaxVoterWeightRecord Realm"
    },
    {
      "code": 6004,
      "name": "InvalidMaxVoterWeightRecordMint",
      "msg": "Invalid MaxVoterWeightRecord Mint"
    },
    {
      "code": 6005,
      "name": "CastVoteIsNotAllowed",
      "msg": "CastVote Is Not Allowed"
    },
    {
      "code": 6006,
      "name": "InvalidVoterWeightRecordRealm",
      "msg": "Invalid VoterWeightRecord Realm"
    },
    {
      "code": 6007,
      "name": "InvalidVoterWeightRecordMint",
      "msg": "Invalid VoterWeightRecord Mint"
    },
    {
      "code": 6008,
      "name": "InvalidTokenOwnerForVoterWeightRecord",
      "msg": "Invalid TokenOwner for VoterWeightRecord"
    },
    {
      "code": 6009,
      "name": "CollectionMustBeVerified",
      "msg": "Collection must be verified"
    },
    {
      "code": 6010,
      "name": "VoterDoesNotOwnNft",
      "msg": "Voter does not own NFT"
    },
    {
      "code": 6011,
      "name": "CollectionNotFound",
      "msg": "Collection not found"
    },
    {
      "code": 6012,
      "name": "MissingMetadataCollection",
      "msg": "Missing Metadata collection"
    },
    {
      "code": 6013,
      "name": "TokenMetadataDoesNotMatch",
      "msg": "Token Metadata doesn't match"
    },
    {
      "code": 6014,
      "name": "InvalidAccountOwner",
      "msg": "Invalid account owner"
    },
    {
      "code": 6015,
      "name": "InvalidTokenMetadataAccount",
      "msg": "Invalid token metadata account"
    },
    {
      "code": 6016,
      "name": "DuplicatedNftDetected",
      "msg": "Duplicated NFT detected"
    },
    {
      "code": 6017,
      "name": "InvalidNftAmount",
      "msg": "Invalid NFT amount"
    },
    {
      "code": 6018,
      "name": "NftAlreadyVoted",
      "msg": "NFT already voted"
    },
    {
      "code": 6019,
      "name": "InvalidProposalForNftVoteRecord",
      "msg": "Invalid Proposal for NftVoteRecord"
    },
    {
      "code": 6020,
      "name": "InvalidTokenOwnerForNftVoteRecord",
      "msg": "Invalid TokenOwner for NftVoteRecord"
    },
    {
      "code": 6021,
      "name": "VoteRecordMustBeWithdrawn",
      "msg": "VoteRecord must be withdrawn"
    },
    {
      "code": 6022,
      "name": "InvalidVoteRecordForNftVoteRecord",
      "msg": "Invalid VoteRecord for NftVoteRecord"
    },
    {
      "code": 6023,
      "name": "VoterWeightRecordMustBeExpired",
      "msg": "VoterWeightRecord must be expired"
    },
    {
      "code": 6024,
      "name": "CannotConfigureCollectionWithVotingProposals",
      "msg": "Cannot configure collection with voting proposals"
    },
    {
      "code": 6025,
      "name": "MustIncludeProposalTransactionForPhaseVotes",
      "msg": "Must include proposals transaction since the proposal ressembles a phase vote"
    }
  ]
};
