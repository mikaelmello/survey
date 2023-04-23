use darling::{FromMeta};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Expr;

use crate::field::FieldSingleContext;

use super::FieldInquireForm;

/// Password prompts are meant for secretive text inputs.
#[derive(Debug, FromMeta, Default)]
#[darling(default)]
pub struct Password {
    /// Message to be presented to the user.
    pub prompt_message: Option<Expr>,
    /// Set the display mode of the text input among hidden, masked and full via the PasswordDisplayMode enum.
    /// * Hidden: default behavior, no UI indicators.
    /// * Masked: behaves like a normal text input, except that all characters of the input are masked to a special character, which is '*' by default but can be customized via RenderConfig.
    /// * Full: behaves like a normal text input, no modifications.
    pub display_mode: Option<Expr>,
    /// By enabling this feature by calling the with_display_toggle_enabled(), you allow the user to toggle between the standard display mode set and the full display mode.
    /// * If you have set the standard display mode to hidden (which is also the default) or masked, the user can press Ctrl+R to change the display mode to Full, and Ctrl+R again to change it back to the standard one.
    /// * Obviously, if you have set the standard display mode to Full, pressing Ctrl+R won't cause any changes.
    pub enable_display_toggle: Option<Expr>,
    /// Message displayed at the line below the prompt.
    pub help_message: Option<Expr>,
    /// Custom formatter in case you need to pre-process the user input before showing it as the final answer.
    /// * By default, it prints eight asterisk characters: ********.
    pub formatter: Option<Expr>,
    /// Custom validators to make sure a given submitted input pass the specified requirements, e.g. not allowing empty inputs or requiring special characters.
    /// * No validators are on by default.
    pub validators: Option<Expr>,
}

impl FieldInquireForm for Password {
    fn generate_inquire_method(
        &self,
        ctx: &FieldSingleContext,
    ) -> Result<TokenStream, Vec<syn::Error>> {
        // contextual parameters
        let fieldname = ctx.ident.to_string();
        let method_name = format_ident!("inquire_{}", fieldname);
        let ty = &ctx.ty;
        let visibility = match ctx.private {
            Some(value) if value => {
                quote! {}
            }
            _ => {
                quote! { pub }
            }
        };

        // generate ident
        let prompt_message = self.prompt_message.as_ref().map_or_else(
            || {
                let prompt_message = format!("What's your {fieldname}?");
                quote! {
                    #prompt_message
                }
            },
            quote::ToTokens::to_token_stream,
        );

        let help_message = self.help_message.as_ref().map_or_else(
            || quote! { inquire::Password::DEFAULT_HELP_MESSAGE },
            |help_message| quote! { Some(#help_message) },
        );

        let display_mode = self.display_mode.as_ref().map_or_else(
            || quote! { inquire::Password::DEFAULT_DISPLAY_MODE },
            |display_mode| {
                quote! {
                    #display_mode
                }
            },
        );

        let enable_display_toggle = self.enable_display_toggle.as_ref().map_or_else(
            || quote! { inquire::Password::DEFAULT_ENABLE_DISPLAY_TOGGLE },
            |enable_display_toggle| quote! { #enable_display_toggle },
        );

        let validators = self.validators.as_ref().map_or_else(
            || quote! { inquire::Password::DEFAULT_VALIDATORS },
            |validators| quote! { #validators },
        );

        let formatter = self.formatter.as_ref().map_or_else(
            || quote! { inquire::Password::DEFAULT_FORMATTER },
            |formatter| quote! { #formatter },
        );

        // Generate method
        Ok(quote! {
            /// Return inquire #fieldname or an [`InquireResult`](inquire::error::InquireResult)
            #visibility fn #method_name(&self) -> inquire::error::InquireResult<#ty> {
                inquire::Password {
                    message: #prompt_message,
                    help_message: #help_message,
                    formatter: #formatter,
                    validators: #validators,
                    enable_display_toggle: #enable_display_toggle,
                    display_mode: #display_mode,
                    render_config: inquire::ui::RenderConfig::default(),
                }
                .prompt()
            }
        })
    }

    fn generate_inquire_method_call(
        &self,
        ctx: &FieldSingleContext,
    ) -> Result<TokenStream, Vec<syn::Error>> {
        let fieldname = format_ident!("{}", ctx.ident.to_string());
        let method_name = format_ident!("inquire_{}", ctx.ident.to_string());
        Ok(quote! {
           self.#fieldname = self.#method_name()?;
        })
    }
}
