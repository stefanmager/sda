use jfs;

use std::path;

use sda_protocol::Identified;
use sda_protocol::{Agent, AgentId, ClerkCandidate, Profile, SignedEncryptionKey, EncryptionKeyId};

use SdaServerResult;
use stores::{BaseStore, AgentsStore};
use jfs_stores::JfsStoreExt;

use itertools::Itertools;

pub struct JfsAgentsStore {
    agents: jfs::Store,
    profiles: jfs::Store,
    encryption_keys: jfs::Store,
}

impl JfsAgentsStore {
    pub fn new<P: AsRef<path::Path>>(prefix: P) -> SdaServerResult<JfsAgentsStore> {
        let agents = prefix.as_ref().join("agents");
        let profiles = prefix.as_ref().join("profiles");
        let encryption_keys = prefix.as_ref().join("encryption_keys");
        Ok(JfsAgentsStore {
            agents: jfs::Store::new(agents.to_str().ok_or("pathbuf to string")?)?,
            profiles: jfs::Store::new(profiles.to_str().ok_or("pathbuf to string")?)?,
            encryption_keys: jfs::Store::new(encryption_keys.to_str().ok_or("pathbuf to string")?)?,
        })
    }
}

impl BaseStore for JfsAgentsStore {
    fn ping(&self) -> SdaServerResult<()> {
        Ok(())
    }
}

impl AgentsStore for JfsAgentsStore {
    fn create_agent(&self, agent: &Agent) -> SdaServerResult<()> {
        self.agents.create(agent)
    }

    fn get_agent(&self, id: &AgentId) -> SdaServerResult<Option<Agent>> {
        self.agents.get_option(id)
    }

    fn upsert_profile(&self, profile: &Profile) -> SdaServerResult<()> {
        self.profiles.upsert_with_id(profile, &profile.owner)
    }

    fn get_profile(&self, owner: &AgentId) -> SdaServerResult<Option<Profile>> {
        self.profiles.get_option(owner)
    }

    fn create_encryption_key(&self, key: &SignedEncryptionKey) -> SdaServerResult<()> {
        self.encryption_keys.create(key)
    }

    fn get_encryption_key(&self,
                          key: &EncryptionKeyId)
                          -> SdaServerResult<Option<SignedEncryptionKey>> {
        self.encryption_keys.get_option(key)
    }

    fn suggest_committee(&self) -> SdaServerResult<Vec<ClerkCandidate>> {
        let keys = self.encryption_keys.all::<SignedEncryptionKey>()?;
        let candidates = keys.into_iter()
            .map(|(_, v)| v)
            .sorted_by(|a, b| a.signer.0.cmp(&b.signer.0))
            .into_iter()
            .group_by(|v| v.signer)
            .into_iter()
            .map(|(k, v)| {
                ClerkCandidate {
                    id: k,
                    keys: v.map(|sek| sek.body.id().clone()).collect(),
                }
            })
            .collect();
        Ok(candidates)
    }
}
