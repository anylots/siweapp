mod app;

use ethers::prelude::*;
use ethers::signers::{LocalWallet, Signer};
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{html, Component, Context, Html};
// use web_sys::Window;

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

enum Msg {
    SetBalance(FetchState<String>),
    SetAccount(String),
    SetPrivateKey(String),
    GetBalance(),
    NoticeSignIn(),
    SignIn(),
    SignInResult(String),
    GetError,
}
struct App {
    account_state: FetchState<String>,
    account: String,
    private_key: String,
    sign_msg: String,
    show_confirm: bool,
    sign_in_result: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            account_state: FetchState::NotFetching,
            account: "0x1".to_owned(),
            private_key: "0x1".to_owned(),
            sign_msg: "0x1".to_owned(),
            show_confirm: false,
            sign_in_result: "0x1".to_owned(),
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetBalance(fetch_state) => {
                self.account_state = fetch_state;
                true
            }
            Msg::GetBalance() => {
                let account = self.account.clone();
                if account == "0x1" {
                    return true;
                }
                ctx.link()
                    .send_message(Msg::SetBalance(FetchState::Fetching));
                ctx.link().send_future(async move {
                    match app::fetch_balance(account).await {
                        Ok(md) => Msg::SetBalance(FetchState::Success(md)),
                        Err(err) => Msg::SetBalance(FetchState::Failed(FetchError::from(
                            JsValue::from_str("fetch fail"),
                        ))),
                    }
                });
                false
            }
            Msg::SetAccount(new_account) => {
                self.account = new_account;
                true
            }

            Msg::SetPrivateKey(private_key) => {
                self.private_key = private_key;
                true
            }

            Msg::NoticeSignIn() => {
                // instantiate the wallet
                let wallet = self.private_key.as_str().parse::<LocalWallet>();
                let address_hex = match wallet {
                    Ok(addr) => hex::encode(H160::as_bytes(&addr.address()).to_vec()),
                    Err(e) => "error".to_string(),
                };
                if address_hex != "error" {
                    self.account = app::eip55(address_hex.clone());
                    let msg = app::create_siwe_str(self.account.clone());
                    self.sign_msg = msg.replace("\n", "<br>");
                    self.show_confirm = true;
                    true
                } else {
                    self.sign_msg = String::from("PrivateKey Invalid");
                    self.show_confirm = false;
                    true
                }
            }

            Msg::SignIn() => {
                let wallet = self.private_key.as_str().parse::<LocalWallet>().unwrap();
                let address = self.account.clone();
                let msg = app::create_siwe_str(address.clone());
                ctx.link().send_future(async move {
                    let signature = wallet.sign_message(msg.as_str()).await.unwrap();
                    let result = app::sign_in(msg, signature, address).await;
                    Msg::SignInResult(result)
                });
                true
            }
            Msg::SignInResult(sign_in_result) => {
                self.sign_in_result = sign_in_result;
                true
            }

            Msg::GetError => {
                ctx.link()
                    .send_message(Msg::SetBalance(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let event: InputEvent = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::SetPrivateKey(target.value())
        });

        html! {
            <main>
                <h1 class="caption">{ "Sign-In With Ethereum" }</h1>
                { self.view_sign_msg(ctx) }
                if self.show_confirm {
                <button onclick={ctx.link().callback(|_| Msg::SignIn())}>{"Confirm SignIn"}</button>
                }
                <div class="privateKey-div">
                    <lable class="privateKey-lable">{"PrivateKey:"}</lable>
                    <input {oninput} class="privateKey form-input" value={self.private_key.clone()} />
                </div>

                <div>
                    <lable>{"Account:"}</lable>
                    <lable> {self.account.clone()} </lable>
                </div>
                <div class="operatiton">
                    <button class="button" onclick={ctx.link().callback(|_| Msg::GetBalance())}>
                        { "Check Balance" }
                    </button>
                    <button class="button signIn" onclick={ctx.link().callback(|_| Msg::NoticeSignIn())}>
                        { "SignIn" }
                    </button>
                </div>
            </main>
        }
    }
}

impl App {
    fn view_sign_msg(&self, ctx: &Context<Self>) -> Html {
        let sign_msg = &self.sign_msg.as_str();
        html! {
            <div class = "pream">
            { for sign_msg.split("<br>").map(|s| html!{<p>{ s }</p>}) }
            </div>
        }
    }
}

fn main() {
    // start siwe-wasm
    yew::Renderer::<App>::new().render();
}
