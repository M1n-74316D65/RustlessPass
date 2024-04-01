use yew::prelude::*;

use crate::fingerprintgen::fingerprint_calculate;
use crate::password_utils::{update_disabled_characters, update_show_state};
use crate::settings::Settings;
use crate::slider::Slider;
use crate::switch::Switch;
use crate::text_input::TextInput;

// Define message enum to handle events
pub enum Msg {
    ChangeSettings(Settings), // Message to change settings
    SetWebsite(String),       // Message to set website value
    SetUsername(String),      // Message to set username value
    SetPassword(String),      // Message to set password value
    GeneratePassword,         // Message to generate password
    ShowInputPassword,        // Message to show/hide password input
}

// Define the main application component
pub struct App {
    settings: Settings,        // Application settings
    website: String,           // Website input value
    username: String,          // Username input value
    password: String,          // Password input value
    new_password: String,      // Newly generated password
    fingerprint: Vec<String>,  // Password fingerprint
    show: u8,                  // State to manage UI
    show_input_password: bool, // Flag to show/hide password input
    disabled: String,          // Flag to disable input fields
}

// Implement default trait for the main application component
impl Default for App {
    fn default() -> Self {
        Self {
            settings: Settings::load(),                    // Load settings
            website: String::new(),                        // Initialize website value
            username: String::new(),                       // Initialize username value
            password: String::new(),                       // Initialize password value
            new_password: "Generate and copy".to_string(), // Initialize new password value
            fingerprint: fingerprint_calculate(""),        // Calculate fingerprint
            show: 0,                                       // Initialize show state
            show_input_password: false,                    // Initialize show_input_password flag
            disabled: String::new(),                       // Initialize disabled flag
        }
    }
}

// Implement component trait for the main application component
impl Component for App {
    type Message = Msg; // Define message type
    type Properties = (); // Define properties type

    // Create function to initialize the component
    fn create(_ctx: &Context<Self>) -> Self {
        Self::default() // Initialize default values
    }

    // Update function to handle messages and update the component
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeSettings(settings) => {
                self.disabled = update_disabled_characters(&settings);
                self.settings = settings; // Update settings
                self.settings.store(); // Store updated settings
                self.show = 0; // Reset password button state
            }
            Msg::SetWebsite(next_website) => {
                // Handle set website message
                self.website = next_website; // Update website value
                self.show = 0; // Reset password button state
            }
            Msg::SetUsername(next_username) => {
                // Handle set username message
                self.username = next_username; // Update username value
                self.show = 0; // Reset password button state
            }
            Msg::SetPassword(next_password) => {
                // Handle set password message
                self.password = next_password; // Update password value
                self.fingerprint = fingerprint_calculate(self.password.clone().as_str()); // Calculate fingerprint
                self.show = 0; // Reset password button state
            }
            Msg::GeneratePassword => {
                // Handle generate password message
                (self.show, self.new_password) = update_show_state(
                    self.show,
                    &self.website,
                    &self.username,
                    &self.password,
                    &self.settings,
                    &self.new_password,
                );
            }

            Msg::ShowInputPassword => {
                // Handle show input password message
                self.show_input_password = !self.show_input_password; // Toggle show input password flag
            }
        };
        true // Return true to indicate successful update
    }

    // View function to render the component
    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_website_change = ctx.link().callback(Msg::SetWebsite); // Callback for website change
        let on_username_change = ctx.link().callback(Msg::SetUsername); // Callback for username change
        let on_password_change = ctx.link().callback(Msg::SetPassword); // Callback for password change
        let on_password_click = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::ShowInputPassword
        }); // Callback for password click
        let on_submit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            Msg::GeneratePassword
        }); // Callback for form submission

        let Self { ref settings, .. } = *self; // Reference to self

        // Macro to generate callback for settings change
        macro_rules! settings_callback {
            ($link:expr, $settings:ident; $key:ident as $ty:ty) => {{
                let settings = $settings.clone();
                $link.callback(move |value| {
                    let mut settings = settings.clone();
                    settings.$key = value as $ty;
                    Msg::ChangeSettings(settings)
                })
            }};
            ($link:expr, $settings:ident; $key:ident) => {
                settings_callback!($link, $settings; $key as u64)
            }
        }

        // Render HTML using Yew's html! macro
        html! {
            <body>
            <nav class="container-fluid">
                <ul>
                <li>
                    <a href="./"><img src="assets/favicon.ico" alt="Description of the image" width="37.7048437335240"
                        height="37.7048437335240"/><strong>{"RustedLessPass"}</strong></a> // Application name
                </li>
                </ul>
                <ul>
                <li>
                    <details class="dropdown">
                    <summary role="button" class="secondary">{"Theme"}</summary> // Theme dropdown
                    <ul dir="rtl">
                        <li><a href="#" data-theme-switcher="auto">{"Auto"}</a></li>
                        <li><a href="#" data-theme-switcher="light">{"Light"}</a></li>
                        <li><a href="#" data-theme-switcher="dark">{"Dark"}</a></li>
                    </ul>
                    </details>
                </li>
                </ul>
            </nav>

            <main class="container">
                <article>
                <div>
                    <hgroup class="title">
                    <h1>{"Stateless Password Manager"}</h1> // Application title
                    <p>{"Remember only one master password to access your passwords at any time, on any device, without the need
                        for
                        syncing."}</p> // Application description
                    </hgroup>
                    <form onsubmit={on_submit}>
                    <TextInput value={self.website.clone()} input_type={"text"} name={"Website"} autocomplete={"off"}
                        on_change={on_website_change} /> // Website input field
                    <TextInput value={self.username.clone()} input_type={"text"} name={"Username"} autocomplete={"email,username"}
                        on_change={on_username_change} /> // Username input field
                    <fieldset role="group">
                        <TextInput value={self.password.clone()} input_type={if self.show_input_password {"text"} else {"password"}}
                        name={"Password"} autocomplete={"current-password"} on_change={on_password_change} /> // Password input field
                        <button
                            style="white-space: nowrap;margin-left: 0rem; padding-left: 0.5rem; padding-right: 0.5rem; align-self: center;"
                            onclick={on_password_click}>
                            <i class={match self.fingerprint.get(0) { Some(s)=> format!("fa fa-fw {}", s),
                            None => String::new(),
                            }} style="margin-right: 0.2rem;"></i>
                            <i class={match self.fingerprint.get(1) { Some(s)=> format!("fa fa-fw {}", s),
                            None => String::new(),
                            }} style="margin-left: 0.2rem;margin-right: 0.2rem;p"></i>
                            <i class={match self.fingerprint.get(2) { Some(s)=> format!("fa fa-fw {}", s),
                            None => String::new(),
                            }} style="margin-left: 0.2rem;"></i>
                        </button> // Password fingerprint icons
                    </fieldset>
                    <fieldset>
                        <nav>

                        <Switch label="a-z" onchange={settings_callback!(ctx.link(), settings; lowercase)}
                            value={settings.lowercase.clone()} value_disabled={if self.disabled.clone()=="a-z" { true } else { false
                            }} /> // Switch for lowercase
                        <Switch label="A-Z" onchange={settings_callback!(ctx.link(), settings; uppercase)}
                            value={settings.uppercase.clone()} value_disabled={if self.disabled.clone()=="A-Z" { true } else { false
                            }} /> // Switch for uppercase
                        <Switch label="0-9" onchange={settings_callback!(ctx.link(), settings; numbers)}
                            value={settings.numbers.clone()} value_disabled={if self.disabled.clone()=="0-9" { true } else { false
                            }} /> // Switch for numbers
                        <Switch label="%!@" onchange={settings_callback!(ctx.link(), settings; symbols)}
                            value={settings.symbols.clone()} value_disabled={if self.disabled.clone()=="%!@" { true } else { false
                            }} /> // Switch for symbols

                        </nav>
                        <div class="grid" style="padding: 0rem;">
                        <Slider label="Size" max=35 min=1 onchange={settings_callback!(ctx.link(), settings; size)}
                            value={settings.size.clone()} /> // Slider for password size
                        <Slider label="Counter" max=100 min=1 onchange={settings_callback!(ctx.link(), settings; counter)}
                            value={settings.counter.clone()} /> // Slider for password counter
                        </div>

                    </fieldset>
                    <button type="submit" class="contrast">{if self.show == 0 {"Generate and copy"} else if self.show == 1
                        {"**************"} else {self.new_password.as_str()}}</button> // Submit button
                    </form>
                </div>
                </article>
            </main>

            <footer class="container-fluid">
                <small>{"Built with "}<a href="https://rust-lang.org" class="secondary">{"Rust"}</a>{", "}<a
                    href="https://github.com/71/lesspass.rs" class="secondary">{"lesspass.rs"}</a>{", "}<a href="https://yew.rs"
                    class="secondary">{"Yew"}</a>{" and "}<a href="https://picocss.com" class="secondary">{"Pico"}</a>{" • "}
                <a href="https://github.com/RustedLessPass/RustedLessPass" class="secondary">{"Source code"}</a></small> // Footer information
            </footer>

            <script src="assets/minimal-theme-switcher.js"></script> // JavaScript for theme switcher
            </body>
        }
    }
}