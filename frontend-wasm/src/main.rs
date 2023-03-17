mod app;

use ethers::signers::{LocalWallet, Signer};
use hex::FromHex;
use std::str::FromStr;

use ethers::core::types::Address;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::Signature;

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{html, Component, Context, Html};
// use web_sys::Window;

const RPC: &str = "http://47.242.179.164:9933";

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

/// fetch_balance.
async fn fetch_balance(url: &'static str, account: String) -> Result<String, FetchError> {
    let provider = Provider::<Http>::try_from("http://47.242.179.164:9933").unwrap();

    let balance_from = provider
        .get_balance(Address::from_str(account.as_str()).unwrap(), None)
        .await
        .unwrap();

    // info!("Hello {}", balance_from.clone());

    Ok(balance_from.to_string())
}

// async fn

enum Msg {
    SetBalance(FetchState<String>),
    SetAccount(String),
    SetPrivateKey(String),
    GetBalance(String),
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

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            account_state: FetchState::NotFetching,
            account: String::from("0x1"),
            private_key: String::from("0x1"),
            sign_msg: String::from("0x1"),
            show_confirm: false,
            sign_in_result: String::from("0x1"),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetBalance(fetch_state) => {
                self.account_state = fetch_state;
                true
            }
            Msg::GetBalance(account) => {
                let account1 = self.account.clone();
                ctx.link().send_future(async move {
                    match fetch_balance(RPC, account1).await {
                        Ok(md) => Msg::SetBalance(FetchState::Success(md)),
                        Err(err) => Msg::SetBalance(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetBalance(FetchState::Fetching));
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
                    Err(e) => "0x1".to_string(),
                };
                self.account = address_hex.clone();
                let msg = app::create_siwe_str(address_hex);
                self.sign_msg = msg.replace("\n", "<br>");
                self.show_confirm = true;
                true
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
                ctx.link().send_future(async {
                    match fetch_balance(RPC, String::from("1")).await {
                        Ok(md) => Msg::SetBalance(FetchState::Success(md)),
                        Err(err) => Msg::SetBalance(FetchState::Failed(err)),
                    }
                });
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

        let value: String = match &self.account_state {
            FetchState::NotFetching => String::from("0x1"),
            FetchState::Fetching => String::from("fetching"),
            FetchState::Success(data) => data.clone(),
            FetchState::Failed(err) => String::from("0x1"),
        };

        html! {
            <main>
            <h1 class = "caption">{ "Sign-In With Ethereum" }</h1>
            {
                self.view_sign_msg(ctx)
            }
            if self.show_confirm {
                <button>{"Confirm SignIn"}</button>
            }
        <div>
          <lable>{"PrivateKey:"}</lable>
          <input {oninput}  class = "privateKey" value={self.private_key.clone()} />
        </div>

         <div>
            <lable>{"Account:"}</lable>
            <lable> {"0x"} {self.account.clone()} </lable>
         </div>
         <div class="operatiton">
           <button onclick={ctx.link().callback(|_| Msg::GetBalance(String::from("0x17155EE3e09033955D272E902B52E0c10cB47A91")))}>
            { "Check Balance" }
           </button>
           <button class="signIn" onclick={ctx.link().callback(|_| Msg::NoticeSignIn())}>
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
        // let parts: Vec<&str> = sign_msg.split("<br>").collect();

        html! {
            <div class = "pream">
            { for sign_msg.split("<br>").map(|s| html!{<p>{ s }</p>}) }
            </div>
        }
    }
}

fn main() {
    // let text = "This is some<br>example<br>text";
    // let parts: Vec<&str> = text.split("<br>").collect();
    // for part in parts {
    //     println!("{}", part);
    // }
    yew::Renderer::<App>::new().render();
}
