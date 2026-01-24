#![no_std]

mod errors;
mod storage;
mod token;

use errors::CrowdfundError;
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};
use storage::{DataKey, ProjectData};

#[contract]
pub struct CrowdfundVaultContract;

#[contractimpl]
impl CrowdfundVaultContract {
    /// Initialize the contract with an admin address
    pub fn initialize(env: Env, admin: Address) -> Result<(), CrowdfundError> {
        // Check if already initialized
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(CrowdfundError::AlreadyInitialized);
        }

        // Require admin authorization
        admin.require_auth();

        // Store admin address
        env.storage().instance().set(&DataKey::Admin, &admin);

        // Initialize project ID counter
        env.storage().instance().set(&DataKey::NextProjectId, &0u64);

        Ok(())
    }

    /// Create a new project
    pub fn create_project(
        env: Env,
        owner: Address,
        name: Symbol,
        target_amount: i128,
        token_address: Address,
    ) -> Result<u64, CrowdfundError> {
        // Check if contract is initialized
        if !env.storage().instance().has(&DataKey::Admin) {
            return Err(CrowdfundError::NotInitialized);
        }

        // Require owner authorization
        owner.require_auth();

        // Validate target amount
        if target_amount <= 0 {
            return Err(CrowdfundError::InvalidAmount);
        }

        // Get next project ID
        let project_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::NextProjectId)
            .unwrap_or(0);

        // Create project data
        let project = ProjectData {
            id: project_id,
            owner: owner.clone(),
            name,
            target_amount,
            token_address: token_address.clone(),
            total_deposited: 0,
            total_withdrawn: 0,
            is_active: true,
        };

        // Store project
        env.storage()
            .persistent()
            .set(&DataKey::Project(project_id), &project);

        // Initialize project balance
        env.storage()
            .persistent()
            .set(&DataKey::ProjectBalance(project_id, token_address), &0i128);

        // Initialize milestone approval status
        env.storage()
            .persistent()
            .set(&DataKey::MilestoneApproved(project_id), &false);

        // Increment project ID counter
        env.storage()
            .instance()
            .set(&DataKey::NextProjectId, &(project_id + 1));

        Ok(project_id)
    }

    /// Deposit funds into a project
    pub fn deposit(
        env: Env,
        user: Address,
        project_id: u64,
        amount: i128,
    ) -> Result<(), CrowdfundError> {
        // Check if contract is initialized
        if !env.storage().instance().has(&DataKey::Admin) {
            return Err(CrowdfundError::NotInitialized);
        }

        // Require user authorization
        user.require_auth();

        // Validate amount
        if amount <= 0 {
            return Err(CrowdfundError::InvalidAmount);
        }

        // Get project
        let mut project: ProjectData = env
            .storage()
            .persistent()
            .get(&DataKey::Project(project_id))
            .ok_or(CrowdfundError::ProjectNotFound)?;

        // Check if project is active
        if !project.is_active {
            return Err(CrowdfundError::ProjectNotActive);
        }

        // Transfer tokens from user to contract
        let contract_address = env.current_contract_address();
        token::transfer(&env, &project.token_address, &user, &contract_address, &amount);

        // Update project balance
        let balance_key = DataKey::ProjectBalance(project_id, project.token_address.clone());
        let current_balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);
        env.storage()
            .persistent()
            .set(&balance_key, &(current_balance + amount));

        // Update project total deposited
        project.total_deposited += amount;
        env.storage()
            .persistent()
            .set(&DataKey::Project(project_id), &project);

        Ok(())
    }

    /// Approve milestone for a project (admin only)
    pub fn approve_milestone(env: Env, admin: Address, project_id: u64) -> Result<(), CrowdfundError> {
        // Check if contract is initialized
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(CrowdfundError::NotInitialized)?;

        // Verify admin identity
        if admin != stored_admin {
            return Err(CrowdfundError::Unauthorized);
        }

        // Require admin authorization
        admin.require_auth();

        // Check if project exists
        if !env
            .storage()
            .persistent()
            .has(&DataKey::Project(project_id))
        {
            return Err(CrowdfundError::ProjectNotFound);
        }

        // Approve milestone
        env.storage()
            .persistent()
            .set(&DataKey::MilestoneApproved(project_id), &true);

        Ok(())
    }

    /// Withdraw funds from a project (owner only, requires milestone approval)
    pub fn withdraw(env: Env, project_id: u64, amount: i128) -> Result<(), CrowdfundError> {
        // Check if contract is initialized
        if !env.storage().instance().has(&DataKey::Admin) {
            return Err(CrowdfundError::NotInitialized);
        }

        // Get project
        let mut project: ProjectData = env
            .storage()
            .persistent()
            .get(&DataKey::Project(project_id))
            .ok_or(CrowdfundError::ProjectNotFound)?;

        // Require owner authorization
        project.owner.require_auth();

        // Check if project is active
        if !project.is_active {
            return Err(CrowdfundError::ProjectNotActive);
        }

        // Validate amount
        if amount <= 0 {
            return Err(CrowdfundError::InvalidAmount);
        }

        // Check milestone approval
        let is_approved: bool = env
            .storage()
            .persistent()
            .get(&DataKey::MilestoneApproved(project_id))
            .unwrap_or(false);

        if !is_approved {
            return Err(CrowdfundError::MilestoneNotApproved);
        }

        // Check balance
        let balance_key = DataKey::ProjectBalance(project_id, project.token_address.clone());
        let current_balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);

        if current_balance < amount {
            return Err(CrowdfundError::InsufficientBalance);
        }

        // Transfer tokens from contract to owner
        let contract_address = env.current_contract_address();
        token::transfer(
            &env,
            &project.token_address,
            &contract_address,
            &project.owner,
            &amount,
        );

        // Update project balance
        env.storage()
            .persistent()
            .set(&balance_key, &(current_balance - amount));

        // Update project total withdrawn
        project.total_withdrawn += amount;
        env.storage()
            .persistent()
            .set(&DataKey::Project(project_id), &project);

        Ok(())
    }

    /// Get project data
    pub fn get_project(env: Env, project_id: u64) -> Result<ProjectData, CrowdfundError> {
        env.storage()
            .persistent()
            .get(&DataKey::Project(project_id))
            .ok_or(CrowdfundError::ProjectNotFound)
    }

    /// Get project balance
    pub fn get_balance(env: Env, project_id: u64) -> Result<i128, CrowdfundError> {
        // Get project to get token address
        let project: ProjectData = env
            .storage()
            .persistent()
            .get(&DataKey::Project(project_id))
            .ok_or(CrowdfundError::ProjectNotFound)?;

        let balance_key = DataKey::ProjectBalance(project_id, project.token_address);
        Ok(env.storage().persistent().get(&balance_key).unwrap_or(0))
    }

    /// Check if milestone is approved for a project
    pub fn is_milestone_approved(env: Env, project_id: u64) -> Result<bool, CrowdfundError> {
        // Check if project exists
        if !env
            .storage()
            .persistent()
            .has(&DataKey::Project(project_id))
        {
            return Err(CrowdfundError::ProjectNotFound);
        }

        Ok(env
            .storage()
            .persistent()
            .get(&DataKey::MilestoneApproved(project_id))
            .unwrap_or(false))
    }

    /// Get admin address
    pub fn get_admin(env: Env) -> Result<Address, CrowdfundError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(CrowdfundError::NotInitialized)
    }
}

#[cfg(test)]
mod test;
