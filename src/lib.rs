pub mod utils;
use {
    crate::utils::*,
    anchor_lang::{
        prelude::*,
        solana_program::{
            program_pack::Pack,
        }
    },
    spl_token::state,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
pub const PRESALE : &str = "presale";
pub const CLIENT_DATA_SIZE : usize = 32+32+32+8+1+1+1;
pub const PRESALE_DATA_SIZE : usize = 32+32+32+32+8+8+8+8+8+8+8+1+1;

#[program]
pub mod solana_anchor {
    use super::*;

    pub fn initialize_presale(
        ctx : Context<InitPresale>,
        _min_allocation : u64,
        _max_allocation : u64,
        _hardcap : u64,
        _token_per_usd_numberator : u64,
        _token_per_usd_denominator : u64,
        _total_percentage_distributed : u64,
        ) -> ProgramResult {
        let presale = &mut ctx.accounts.presale;
        let presale_pot : state::Account = state::Account::unpack_from_slice(&ctx.accounts.presale_pot.data.borrow())?;
        // let token_being_raised : state::Mint = state::Mint::unpack_from_slice(&ctx.accounts.token_being_raised.borrow())?;
        // let token_for_sale : state::Mint = state::Mint::unpack_from_slice(&ctx.accounts.token_for_sale.borrow())?;
        
        if presale_pot.mint != *ctx.accounts.token_being_raised.key {
            return Err(PresaleError::NotMatchMintAddress.into());
        }
        if presale_pot.owner != ctx.accounts.authority.key() {
            return Err(PresaleError::NotMatchOwnerAddress.into());
        }
        presale.authority = ctx.accounts.authority.key();
        presale.presale_pot = *ctx.accounts.presale_pot.key;
        presale.token_for_sale = *ctx.accounts.token_for_sale.key;
        presale.token_being_raised = *ctx.accounts.token_being_raised.key;
        presale.min_allocation = _min_allocation;
        presale.max_allocation = _max_allocation;
        presale.hardcap = _hardcap;
        presale.token_per_usd_numerator = _token_per_usd_numberator;
        presale.token_per_usd_denominator=_token_per_usd_denominator;
        presale.total_percentage_distributed = _total_percentage_distributed;
        presale.is_active = false;
        presale.is_whitelist = true;
        presale.total_raised = 0;
        Ok(())
    }

    pub fn initialize_client(
        ctx : Context<InitClient>,
        ) -> ProgramResult{
        let presale = &mut ctx.accounts.presale;
        let client = &mut ctx.accounts.client;
        let client_pot : state::Account = state::Account::unpack_from_slice(&ctx.accounts.client_pot.data.borrow())?;
        if client_pot.mint != presale.token_for_sale {
            return Err(PresaleError::NotMatchMintAddress.into());
        }
        if client_pot.owner != ctx.accounts.authority.key() {
            return Err(PresaleError::NotMatchOwnerAddress.into());
        }
        client.owner = ctx.accounts.authority.key();
        client.presale = ctx.accounts.presale.key();
        client.client_pot = *ctx.accounts.client_pot.key;
        client.amount = 0;
        client.is_whitelisted = false;
        client.is_initialized = true;
        client.already_paid = false;
        Ok(())
    }

    pub fn add_to_whitelist(ctx : Context<AddToWhitelist>) -> ProgramResult {
        let presale = &mut ctx.accounts.presale;
        let client = &mut ctx.accounts.client;
        if client.presale != presale.key() {
            return Err(PresaleError::InvalidPresaleAccount.into());
        }
        if client.owner != *ctx.accounts.member.key {
            return Err(PresaleError::InvalidClientOwner.into());
        }
        if presale.is_active {
            return Err(PresaleError::AlreadyStarted.into());
        }
        client.is_whitelisted = true;
        Ok(())
    }

    pub fn start_presale(ctx : Context<StartPresale>) -> ProgramResult {
        let presale = &mut ctx.accounts.presale;
        if presale.is_active {
            return Err(PresaleError::AlreadyStarted.into());
        }
        presale.is_active=true;
        Ok(())
    }

    pub fn stop_presale(ctx : Context<StopPresale>) -> ProgramResult {
        let presale = &mut ctx.accounts.presale;
        if !presale.is_active {
            return Err(PresaleError::AlreadyStopped.into());
        }
        presale.is_active=false;
        Ok(())
    }

    pub fn stop_whitelist(ctx : Context<StopWhitelist>) -> ProgramResult {
        let presale = &mut ctx.accounts.presale;
        if !presale.is_whitelist {
            return Err(PresaleError::AlreadyStopped.into());
        }
        presale.is_whitelist=false;
        Ok(())
    }

    pub fn buy(ctx : Context<Buy>, amount : u64) -> ProgramResult {
        let client = &mut ctx.accounts.client;

        let bidder_token : state::Account = state::Account::unpack_from_slice(&ctx.accounts.bidder_token.data.borrow())?;
        let presale_pot : state::Account = state::Account::unpack_from_slice(&ctx.accounts.presale_pot.data.borrow())?;

        if bidder_token.mint != *ctx.accounts.mint.key {
            return Err(PresaleError::NotMatchMintAddress.into());
        }

        if presale_pot.mint != *ctx.accounts.mint.key {
            return Err(PresaleError::NotMatchMintAddress.into());
        }

        if bidder_token.owner != *ctx.accounts.bidder.key {
            return Err(PresaleError::NotMatchOwnerAddress.into());
        }

        if client.owner != *ctx.accounts.bidder.key {
            return Err(PresaleError::NotMatchOwnerAddress.into());
        }

        if client.presale != ctx.accounts.presale.key(){
            return Err(PresaleError::NotMatchPresaleAddress.into());
        }

        ///////////////////////////////////////////////////////////////////
        let presale = &mut ctx.accounts.presale;
        if presale.presale_pot != *ctx.accounts.presale_pot.key {
            return Err(PresaleError::NotMatchPresalePotAddress.into());
        }

        if presale.token_being_raised != *ctx.accounts.mint.key {
            return Err(PresaleError::NotMatchMintAddress.into());
        }

        if !presale.is_active {
            return Err(PresaleError::NotActiveYet.into());
        }

        if amount < presale.min_allocation || amount > presale.max_allocation {
            return Err(PresaleError::InvalidAmount.into());
        }

        // if bidder_token.amount.saturating_sub(amount) < 0 {
        //     return Err(PresaleError::BalanceTooLow.into());
        // }

        if presale.total_raised > presale.hardcap {
            return Err(PresaleError::HardcapReached.into());
        }

        if (presale.total_raised + amount) > presale.hardcap {
            return Err(PresaleError::WillOverHardcap.into());
        }

        if (client.amount + amount) > presale.max_allocation {
            return Err(PresaleError::MoreThanMaxAllocation.into());
        }

        if presale.is_whitelist==true && client.is_whitelisted==false {
            return Err(PresaleError::NotWhitelisted.into());
        }

        spl_token_transfer_without_seed(
            TokenTransferParamsWithoutSeed{
                source : ctx.accounts.bidder_token.clone(),
                destination : ctx.accounts.presale_pot.clone(),
                authority : ctx.accounts.bidder.clone(),
                token_program : ctx.accounts.token_program.clone(),
                amount : amount,
            }
        )?;
        // client.already_paid = false;
        client.amount = client.amount + amount;
        presale.total_raised = presale.total_raised + amount;

        Ok(())
    }

    pub fn distribute_token(ctx : Context<DistributeToken>, percentage_of_amount_owed : u64) -> ProgramResult {
        let client = &mut ctx.accounts.client;

        let client_pot : state::Account = state::Account::unpack_from_slice(&ctx.accounts.client_pot.data.borrow())?;
        let auth_token : state::Account = state::Account::unpack_from_slice(&ctx.accounts.auth_token.data.borrow())?;

        if client_pot.mint != *ctx.accounts.mint.key {
            return Err(PresaleError::NotMatchMintAddress.into());
        }

        if auth_token.mint != *ctx.accounts.mint.key {
            return Err(PresaleError::NotMatchMintAddress.into());
        }

        if auth_token.owner != *ctx.accounts.authority.key {
            return Err(PresaleError::NotMatchOwnerAddress.into());
        }

        if client.presale != ctx.accounts.presale.key(){
            return Err(PresaleError::NotMatchPresaleAddress.into());
        }

        // if client.already_paid {
        //     return Err(PresaleError::AlreadyPaied.into());
        // }

        ////////////////////////////////////////////////////////////////////
        let presale = &mut ctx.accounts.presale;

        if presale.token_for_sale != *ctx.accounts.mint.key {
            return Err(PresaleError::NotMatchPresalePotAddress.into());
        }

        // if presale.is_active {
        //     return Err(PresaleError::NotStoppedYet.into());
        // }

        if client.amount==0 {
            return Err(PresaleError::InvalidAmount.into());
        }

        if (percentage_of_amount_owed + presale.total_percentage_distributed) > 100 {
            return Err(PresaleError::AlreadyDistributedOverflow.into());
        }

        let real_amount = ((client.amount as f64) * (presale.token_per_usd_numerator as f64) / (presale.token_per_usd_denominator as f64) / (100.0 as f64) * (percentage_of_amount_owed as f64)) as u64 ;
        
        spl_token_transfer_without_seed(
            TokenTransferParamsWithoutSeed{
                source : ctx.accounts.auth_token.clone(),
                destination : ctx.accounts.client_pot.clone(),
                authority : ctx.accounts.authority.clone(),
                token_program : ctx.accounts.token_program.clone(),
                amount : real_amount,
            }
        )?;

        client.amount = 0;
        // client.already_paid = true;
        Ok(())
    }

    pub fn set_authority(ctx : Context<SetAuthority>) -> ProgramResult {
        let presale = &mut ctx.accounts.presale;

        if presale.presale_pot != *ctx.accounts.presale_pot.key {
            return Err(PresaleError::NotMatchPresalePotAddress.into());
        }

        spl_token_set_authority(
            TokenSetAuthorityParams{
                authority : ctx.accounts.authority.clone(),
                new_authority : ctx.accounts.new_authority.clone(),
                account : ctx.accounts.presale_pot.clone(),
                token_program : ctx.accounts.token_program.clone(),
            }
        )?;

        presale.authority = *ctx.accounts.new_authority.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(mut, signer)]
    authority : AccountInfo<'info>,

     #[account(mut)]
    new_authority : AccountInfo<'info>,

    #[account(mut, owner=spl_token::id())]
    presale_pot : AccountInfo<'info>,

    #[account(mut,has_one = authority)]
    presale : ProgramAccount<'info,PresaleData>,

    #[account(address=spl_token::id())]
    token_program : AccountInfo<'info>,    
}

#[derive(Accounts)]
pub struct DistributeToken<'info> {
    #[account(mut, signer)]
    authority : AccountInfo<'info>,

    #[account(mut, owner=spl_token::id())]
    auth_token : AccountInfo<'info>,

    #[account(mut, owner=spl_token::id())]
    client_pot : AccountInfo<'info>,

    #[account(mut, owner=spl_token::id())]
    mint : AccountInfo<'info>,

    #[account(mut, has_one=authority)]
    presale : ProgramAccount<'info,PresaleData>,
    
    #[account(mut)]
    client : ProgramAccount<'info,ClientData>,

    #[account(address=spl_token::id())]
    token_program : AccountInfo<'info>, 
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut,signer)]
    bidder : AccountInfo<'info>,

    #[account(mut, owner=spl_token::id())]
    bidder_token : AccountInfo<'info>,

    #[account(mut, owner=spl_token::id())]
    presale_pot : AccountInfo<'info>,

    #[account(mut, owner=spl_token::id())]
    mint : AccountInfo<'info>,

    #[account(mut)]
    presale : ProgramAccount<'info,PresaleData>,
    
    #[account(mut)]
    client : ProgramAccount<'info,ClientData>,

    #[account(address=spl_token::id())]
    token_program : AccountInfo<'info>,    
}

#[derive(Accounts)]
pub struct StopWhitelist<'info> {
    #[account(mut,has_one = authority)]
    presale : ProgramAccount<'info,PresaleData>,
    authority : Signer<'info>,
}

#[derive(Accounts)]
pub struct StopPresale<'info> {
    #[account(mut, has_one = authority)]
    presale : ProgramAccount<'info,PresaleData>,
    authority : Signer<'info>,
}

#[derive(Accounts)]
pub struct StartPresale<'info> {
    #[account(mut, has_one = authority)]
    presale : ProgramAccount<'info,PresaleData>,
    authority : Signer<'info>,
}

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(mut,has_one = authority)]
    presale : ProgramAccount<'info,PresaleData>,
    
    #[account(mut)]
    client : ProgramAccount<'info,ClientData>,
    
    #[account(mut)]
    authority : Signer<'info>,
    
    member : AccountInfo<'info>  
}

#[derive(Accounts)]
pub struct InitClient<'info> {
    #[account(init, payer = authority, space=8+CLIENT_DATA_SIZE)]
    client : ProgramAccount<'info,ClientData>,
    
    #[account(mut)]
    authority : Signer<'info>,
    
    #[account(mut)]
    presale : ProgramAccount<'info,PresaleData>,
    
    #[account(mut, owner=spl_token::id())]
    client_pot : AccountInfo<'info>,

    system_program : Program<'info,System>
}

#[derive(Accounts)]
pub struct InitPresale<'info> {
    #[account(init, payer = authority, space=8+PRESALE_DATA_SIZE)]
    presale : ProgramAccount<'info,PresaleData>,
    #[account(mut)]
    authority : Signer<'info>,
    
    #[account(mut, owner=spl_token::id())]
    presale_pot : AccountInfo<'info>,
    
    #[account(mut, owner=spl_token::id())]
    token_for_sale : AccountInfo<'info>,
    
    #[account(mut, owner=spl_token::id())]
    token_being_raised : AccountInfo<'info>,
    
    system_program : Program<'info,System>,
}

#[account]
pub struct PresaleData {
    pub authority : Pubkey,
    pub presale_pot : Pubkey,
    pub token_for_sale : Pubkey,
    pub token_being_raised : Pubkey,
    pub min_allocation : u64,
    pub max_allocation : u64,
    pub hardcap : u64,
    pub token_per_usd_numerator : u64,
    pub token_per_usd_denominator : u64,
    pub total_raised : u64,
    pub total_percentage_distributed : u64,
    pub is_active : bool,
    pub is_whitelist : bool,
}

#[account]
pub struct ClientData {
    pub presale : Pubkey,
    pub owner : Pubkey,
    pub client_pot : Pubkey,
    pub amount : u64,
    pub is_whitelisted : bool,
    pub already_paid : bool,
    pub is_initialized : bool,
}

#[error]
pub enum PresaleError {
    #[msg("Account does not have correct owner")]
    IncorrectOwner,

    #[msg("Derived key is invalid")]
    DerivedKeyInvalid,

    #[msg("Invalid authority")]
    InvalidAuthority,

    #[msg("Presale has already started")]
    AlreadyStarted,

    #[msg("Data type mismatch")]
    DataTypeMismatch,

    #[msg("Already stopped")]
    AlreadyStopped,

    #[msg("Invalid client owner")]
    InvalidClientOwner,

    #[msg("Invalid presale account")]
    InvalidPresaleAccount,

    #[msg("Invalid token program")]
    InvalidTokenProgram,

    #[msg("Not match presale address")]
    NotMatchPresale,

    #[msg("Preslae is not active yet")]
    NotActiveYet,

    #[msg("Amount is invalid")]
    InvalidAmount,

    #[msg("Not match token address")]
    NotMatchTokenAddress,

    #[msg("Balance too low")]
    BalanceTooLow,

    #[msg("Hardcap has been reached")]
    HardcapReached,

    #[msg("You will be going over the hardcap")]
    WillOverHardcap,

    #[msg("You cant buy more than the max allocation")]
    MoreThanMaxAllocation,

    #[msg("You are not whitelisted")]
    NotWhitelisted,

    #[msg("Token transfer failed")]
    TokenTransferFailed,

    #[msg("Already distributed 100% of tokens")]
    AlreadyDistributedOverflow,

    #[msg("Not match mint address")]
    NotMatchMintAddress,

    #[msg("Not match owner address")]
    NotMatchOwnerAddress,

    #[msg("Not match presale address")]
    NotMatchPresaleAddress,

    #[msg("Not match presale pot address")]
    NotMatchPresalePotAddress,

    #[msg("Already paid")]
    AlreadyPaied,

    #[msg("Presale is not stopped yet")]
    NotStoppedYet,

    #[msg("Token set authority failed")]
    TokenSetAuthorityFailed,
}