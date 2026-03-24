#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, BytesN, Env, String,
    Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidInput = 1,
    LicenseAlreadyRegistered = 2,
    InvalidOrgType = 3,
    AlreadyInitialized = 4,
    Unauthorized = 5,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrgType {
    BloodBank,
    Hospital,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Role {
    BloodBank,
    Hospital,
    Admin,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Organization {
    pub id: Address,
    pub org_type: OrgType,
    pub name: String,
    pub license_number: String,
    pub verified: bool,
    pub verified_timestamp: Option<u64>,
    pub rating: u32,
    pub total_ratings: u32,
    pub location_hash: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct OrganizationRegistered {
    pub org_id: Address,
    pub org_type: OrgType,
    pub name: String,
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum DataKey {
    Org(Address),
    License(String),
    Docs(Address),
    Role(Address),
    OrgCounter,
    Admin,
}

#[contract]
pub struct IdentityContract;

#[contractimpl]
impl IdentityContract {
    /// Initialize the contract with an admin
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        Self::grant_role(env, admin, Role::Admin);
        Ok(())
    }

    /// Register a new organization
    pub fn register_organization(
        env: Env,
        owner: Address,
        org_type: OrgType,
        name: String,
        license_number: String,
        location_hash: BytesN<32>,
        document_hashes: Vec<BytesN<32>>,
    ) -> Result<Address, Error> {
        owner.require_auth();

        // Validate data
        if name.len() == 0 || license_number.len() == 0 {
            return Err(Error::InvalidInput);
        }

        // Check if license number is unique
        let license_key = DataKey::License(license_number.clone());
        if env.storage().persistent().has(&license_key) {
            return Err(Error::LicenseAlreadyRegistered);
        }

        // Generate organization ID (using owner address as ID)
        let org_id = owner.clone();

        // Create organization
        let organization = Organization {
            id: org_id.clone(),
            org_type: org_type.clone(),
            name: name.clone(),
            license_number: license_number.clone(),
            verified: false,
            verified_timestamp: None,
            rating: 0,
            total_ratings: 0,
            location_hash,
        };

        // Store organization
        env.storage()
            .persistent()
            .set(&DataKey::Org(org_id.clone()), &organization);

        // Store license mapping
        env.storage().persistent().set(&license_key, &org_id);

        // Store documents
        env.storage()
            .persistent()
            .set(&DataKey::Docs(org_id.clone()), &document_hashes);

        // Assign role based on type
        let role = match org_type {
            OrgType::BloodBank => Role::BloodBank,
            OrgType::Hospital => Role::Hospital,
        };
        Self::grant_role(env.clone(), org_id.clone(), role);

        // Increment counter
        Self::increment_counter(&env, DataKey::OrgCounter);

        // Emit event
        env.events().publish(
            (symbol_short!("org_reg"),),
            OrganizationRegistered {
                org_id: org_id.clone(),
                org_type,
                name,
            },
        );

        Ok(org_id)
    }

    /// Internal helper to grant a role to an address
    pub fn grant_role(env: Env, address: Address, role: Role) {
        env.storage().persistent().set(&DataKey::Role(address), &role);
    }

    /// Get the role of an address
    pub fn get_role(env: Env, address: Address) -> Option<Role> {
        env.storage().persistent().get(&DataKey::Role(address))
    }

    /// Internal helper to increment a counter
    fn increment_counter(env: &Env, key: DataKey) -> u32 {
        let mut count: u32 = env.storage().instance().get(&key).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&key, &count);
        count
    }

    /// Get organization by ID
    pub fn get_organization(env: Env, org_id: Address) -> Option<Organization> {
        env.storage().persistent().get(&DataKey::Org(org_id))
    }
}

mod test;
