//! P4Info helper

use std::{collections::HashMap, ops::Deref};

use p4runtime::p4::config::v1 as p4_cfg_v1;

/// P4Info Helper
#[derive(Clone, Debug, Default)]
pub struct P4Info {
    /// Internal P4Info object
    p4info: Option<p4_cfg_v1::P4Info>,

    /// table_name, id mapping
    table_id_map: HashMap<String, u32>,
    /// (table_name, match_field_name), id mapping
    table_match_field_id_map: HashMap<(String, String), u32>,
    /// action_name, id mapping
    action_id_map: HashMap<String, u32>,
    /// action_profile_name, id mapping
    action_profile_id_map: HashMap<String, u32>,
    /// counter_name, id mapping
    counter_id_map: HashMap<String, u32>,
    /// direct_counter_name, id mapping
    direct_counter_id_map: HashMap<String, u32>,
    /// meter_name, id mapping
    meter_id_map: HashMap<String, u32>,
    /// direct_meter_name, id mapping
    direct_meter_id_map: HashMap<String, u32>,
    /// value_set_name, id mapping
    value_set_id_map: HashMap<String, u32>,
    /// register_name, id mapping
    register_id_map: HashMap<String, u32>,
    /// digest_name, id mapping
    digest_id_map: HashMap<String, u32>,
}

impl AsRef<p4_cfg_v1::P4Info> for P4Info {
    fn as_ref(&self) -> &p4_cfg_v1::P4Info {
        self.p4info.as_ref().expect("P4Info not loaded")
    }
}

impl Deref for P4Info {
    type Target = p4_cfg_v1::P4Info;

    fn deref(&self) -> &Self::Target {
        self.p4info.as_ref().expect("P4Info not loaded")
    }
}

impl P4Info {
    /// Load P4Info to P4Info Helper
    pub fn load(&mut self, p4info: p4_cfg_v1::P4Info) {
        self.p4info = Some(p4info);

        self.table_id_map = self
            .p4info
            .as_ref()
            .unwrap()
            .tables
            .iter()
            .flat_map(|table| {
                [
                    (
                        table.preamble.as_ref().unwrap().name.clone(),
                        table.preamble.as_ref().unwrap().id,
                    ),
                    (
                        table.preamble.as_ref().unwrap().alias.clone(),
                        table.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.table_match_field_id_map = self
            .p4info
            .as_ref()
            .unwrap()
            .tables
            .iter()
            .flat_map(|table| {
                table.match_fields.iter().flat_map(|match_field| {
                    [
                        (
                            (
                                table.preamble.as_ref().unwrap().name.clone(),
                                match_field.name.clone(),
                            ),
                            match_field.id,
                        ),
                        (
                            (
                                table.preamble.as_ref().unwrap().alias.clone(),
                                match_field.name.clone(),
                            ),
                            match_field.id,
                        ),
                    ]
                })
            })
            .collect();

        self.action_id_map = self
            .p4info
            .as_ref()
            .unwrap()
            .actions
            .iter()
            .flat_map(|action| {
                [
                    (
                        action.preamble.as_ref().unwrap().name.clone(),
                        action.preamble.as_ref().unwrap().id,
                    ),
                    (
                        action.preamble.as_ref().unwrap().alias.clone(),
                        action.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.action_profile_id_map = self
            .p4info
            .as_ref()
            .unwrap()
            .action_profiles
            .iter()
            .flat_map(|action_profile| {
                [
                    (
                        action_profile.preamble.as_ref().unwrap().name.clone(),
                        action_profile.preamble.as_ref().unwrap().id,
                    ),
                    (
                        action_profile.preamble.as_ref().unwrap().alias.clone(),
                        action_profile.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.counter_id_map = self
            .p4info
            .as_ref()
            .unwrap()
            .counters
            .iter()
            .flat_map(|counter| {
                [
                    (
                        counter.preamble.as_ref().unwrap().name.clone(),
                        counter.preamble.as_ref().unwrap().id,
                    ),
                    (
                        counter.preamble.as_ref().unwrap().alias.clone(),
                        counter.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.direct_counter_id_map = self
            .p4info
            .as_ref()
            .unwrap()
            .direct_counters
            .iter()
            .flat_map(|direct_counter| {
                [
                    (
                        direct_counter.preamble.as_ref().unwrap().name.clone(),
                        direct_counter.preamble.as_ref().unwrap().id,
                    ),
                    (
                        direct_counter.preamble.as_ref().unwrap().alias.clone(),
                        direct_counter.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.meter_id_map = self
            .p4info
            .as_ref()
            .unwrap()
            .meters
            .iter()
            .flat_map(|meter| {
                [
                    (
                        meter.preamble.as_ref().unwrap().name.clone(),
                        meter.preamble.as_ref().unwrap().id,
                    ),
                    (
                        meter.preamble.as_ref().unwrap().alias.clone(),
                        meter.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.direct_meter_id_map = self
            .p4info
            .as_ref()
            .unwrap()
            .direct_meters
            .iter()
            .flat_map(|direct_meter| {
                [
                    (
                        direct_meter.preamble.as_ref().unwrap().name.clone(),
                        direct_meter.preamble.as_ref().unwrap().id,
                    ),
                    (
                        direct_meter.preamble.as_ref().unwrap().alias.clone(),
                        direct_meter.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.value_set_id_map = self
            .p4info
            .as_ref()
            .unwrap()
            .value_sets
            .iter()
            .flat_map(|value_set| {
                [
                    (
                        value_set.preamble.as_ref().unwrap().name.clone(),
                        value_set.preamble.as_ref().unwrap().id,
                    ),
                    (
                        value_set.preamble.as_ref().unwrap().alias.clone(),
                        value_set.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.register_id_map = self
            .p4info
            .as_ref()
            .unwrap()
            .registers
            .iter()
            .flat_map(|register| {
                [
                    (
                        register.preamble.as_ref().unwrap().name.clone(),
                        register.preamble.as_ref().unwrap().id,
                    ),
                    (
                        register.preamble.as_ref().unwrap().alias.clone(),
                        register.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.digest_id_map = self
            .p4info
            .as_ref()
            .unwrap()
            .digests
            .iter()
            .flat_map(|digest| {
                [
                    (
                        digest.preamble.as_ref().unwrap().name.clone(),
                        digest.preamble.as_ref().unwrap().id,
                    ),
                    (
                        digest.preamble.as_ref().unwrap().alias.clone(),
                        digest.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();
    }

    /// Find table id by table name
    ///
    /// If table name not found, return 0 (wildcard)
    pub fn table_id(&self, table_name: &str) -> u32 {
        *self.table_id_map.get(table_name).unwrap_or(&0)
    }

    /// Find table match field id by table name and match field name
    ///
    /// If not found, return 0 (wildcard)
    pub fn table_match_field_id(&self, table_name: &str, match_field_name: &str) -> u32 {
        *self
            .table_match_field_id_map
            .get(&(table_name.to_string(), match_field_name.to_string()))
            .unwrap_or(&0)
    }

    /// Find action id by action name
    ///
    /// If not found, return 0
    pub fn action_id(&self, action_name: &str) -> u32 {
        *self.action_id_map.get(action_name).unwrap_or(&0)
    }

    /// Find action profile id by action profile name
    ///
    /// If not found, return 0
    pub fn action_profile_id(&self, action_profile_name: &str) -> u32 {
        *self
            .action_profile_id_map
            .get(action_profile_name)
            .unwrap_or(&0)
    }

    /// Find counter id by counter name
    ///
    /// If not found, return 0
    pub fn counter_id(&self, counter_name: &str) -> u32 {
        *self.counter_id_map.get(counter_name).unwrap_or(&0)
    }

    /// Find direct counter id by direct counter name
    ///
    /// If not found, return 0
    pub fn direct_counter_id(&self, direct_counter_name: &str) -> u32 {
        *self
            .direct_counter_id_map
            .get(direct_counter_name)
            .unwrap_or(&0)
    }

    /// Find meter id by meter name
    ///
    /// If not found, return 0
    pub fn meter_id(&self, meter_name: &str) -> u32 {
        *self.meter_id_map.get(meter_name).unwrap_or(&0)
    }

    /// Find direct meter id by direct meter name
    ///
    /// If not found, return 0
    pub fn direct_meter_id(&self, direct_meter_name: &str) -> u32 {
        *self
            .direct_meter_id_map
            .get(direct_meter_name)
            .unwrap_or(&0)
    }

    /// Find value set id by value set name
    ///
    /// If not found, return 0
    pub fn value_set_id(&self, value_set_name: &str) -> u32 {
        *self.value_set_id_map.get(value_set_name).unwrap_or(&0)
    }

    /// Find register id by register name
    ///
    /// If not found, return 0
    pub fn register_id(&self, register_name: &str) -> u32 {
        *self.register_id_map.get(register_name).unwrap_or(&0)
    }

    /// Find digest id by digest name
    ///
    /// If not found, return 0
    pub fn digest_id(&self, digest_name: &str) -> u32 {
        *self.digest_id_map.get(digest_name).unwrap_or(&0)
    }
}
