mod context;

use anchor_lang::prelude::*;
use std::io::{self, Write};
use std::cmp;
use std::cmp::max;
use solana_program::program_memory::sol_memcpy;
declare_id!("FJ6ZsJXSxXQjrTh7N9RetoFi8AyfLKYQhQPnvAfyhwVU");

const USER_NAME_LENGTH: usize = 100;
const USER_URL_LENGTH: usize = 225;
const USER_DESCRIPTION_LENGTH: usize = 225;
const USER_STATUS_LENGTH: usize = 225;

const VIDEO_URL_LENGTH: usize = 225;
const VIDEO_DESCRIPTION_LENGTH: usize = 225;

#[program]
pub mod testerdis {
    use crate::context::{QueueHeader, UserProfileVault};
    use super::*;

    pub fn initialize_user<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, InitializeUser<'info>>,
        name: String,
        age: String,
        gender: String,
        profile_url: String,
        description: String,
        country: String,
    ) -> Result<()> {
        // let user_profile = &mut ctx.accounts.user_profile;
        // user_profile.authority = ctx.accounts.authority.key();
        // user_profile.wallet_address = ctx.accounts.authority.key();
        // user_profile.name = name;
        // user_profile.age = age;
        // user_profile.gender = gender;
        // user_profile.total_friend = 0;
        // user_profile.status_index = 0;
        // user_profile.status_count = 0;
        // user_profile.video_index = 0;
        // user_profile.init_time = ctx.accounts.clock.unix_timestamp;
        // user_profile.profile_url = profile_url;
        // user_profile.country = country;
        // user_profile.description = description;

        let user_profile = UserProfileVault {
            authority: ctx.accounts.authority.key(),
            name: name,
            age: age,
            status_index: 0,
            status_count: 0,
            video_index: 0,
            gender: gender,
            profile_url: profile_url,
            wallet_address: ctx.accounts.authority.key(),
            total_friend: 0,
            country: country,
            description: description,
            init_time: ctx.accounts.clock.unix_timestamp,
            withdraw_queue_header: ctx.accounts.withdraw_queue_header.key(),
        };
        let info = ctx.accounts.user_profile_vault.to_account_info();
        let mut data = info.try_borrow_mut_data()?;
        let dst: &mut [u8] = &mut data;
        let mut writer = BpfWriter::new(dst); // notice this
        user_profile.try_serialize(&mut writer)?;
        Ok(())
    }

    #[derive(Accounts)]
    #[instruction()]
    pub struct InitializeUser<'info> {
        #[account(
            init,
            space = 8 + QueueHeader::INIT_SPACE,
            seeds = [b"WITHDRAW_QUEUE_SEED"],
            bump,
            payer = authority
        )]
        pub withdraw_queue_header: Box<Account<'info, QueueHeader>>,

        #[account(

            init,
            seeds = [b"USER_TAG"],
            bump,
            payer = authority,
            space = 8 + std::mem::size_of::< UserProfileVault > () + USER_NAME_LENGTH + USER_URL_LENGTH + USER_DESCRIPTION_LENGTH,
        )]
        /// CHECK : This is Safe
        pub user_profile_vault: UncheckedAccount<'info>,

        #[account(mut)]
        pub authority: Signer<'info>,

        pub system_program: Program<'info, System>,

        pub clock: Sysvar<'info, Clock>,

    }
}

#[derive(Debug, Default)]
pub struct BpfWriter<T> {
    inner: T,
    pos: u64,
}

impl<T> BpfWriter<T> {
    pub fn new(inner: T) -> Self {
        Self { inner, pos: 0 }
    }
}

impl Write for BpfWriter<&mut [u8]> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.pos >= self.inner.len() as u64 {
            return Ok(0);
        }

        let amt = cmp::min(
            self.inner.len().saturating_sub(self.pos as usize),
            buf.len(),
        );
        sol_memcpy(&mut self.inner[(self.pos as usize)..], buf, amt);
        self.pos += amt as u64;
        Ok(amt)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        if self.write(buf)? == buf.len() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "failed to write whole buffer",
            ))
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

