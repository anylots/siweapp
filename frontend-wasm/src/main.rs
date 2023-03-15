mod app;

use std::str::FromStr;

use ethers::core::types::Address;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
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

enum Msg {
    SetBalance(FetchState<String>),
    SetAccount(String),
    GetBalance(String),
    NoticeSignIn(String),
    GetError,
}
struct App {
    account_state: FetchState<String>,
    account: String,
    sign_msg: String,

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            account_state: FetchState::NotFetching,
            account: String::from("0x1"),
            sign_msg: String::from("0x1"),
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
            Msg::NoticeSignIn(new_account) => {
                let msg = app::createSiweStr( self.account.clone());
                self.sign_msg = msg;
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
            Msg::SetAccount(target.value())
        });

        let value: String = match &self.account_state {
            FetchState::NotFetching => String::from("0x1"),
            FetchState::Fetching => String::from("fetching"),
            FetchState::Success(data) => data.clone(),
            FetchState::Failed(err) => String::from("0x1"),
        };

        // let mut msg = app::createSiweStr("0x63F9725f107358c9115BC9d86c72dD5823E9B1E6".to_string());
        // msg = msg.replace("\n", "</br>");

        html! {
           <main>
           <h1 class = "caption">{ "Sign-In With Ethereum" }</h1>
           <div class = "pream">
           <lable>{self.sign_msg.clone()}</lable>
           </div>

           <div>
             <lable>{"PrivateKey:"}</lable>
             <input class = "privateKey"  />
           </div>
            <div>
               <lable>{"Account:"}</lable>
               <input {oninput} value={self.account.clone()} />
            </div>
            <div class="operatiton">
              <button onclick={ctx.link().callback(|_| Msg::GetBalance(String::from("0x17155EE3e09033955D272E902B52E0c10cB47A91")))}>
               { "Get Balance" }
              </button>
              <button onclick={ctx.link().callback(|_| Msg::NoticeSignIn(String::from("0x17155EE3e09033955D272E902B52E0c10cB47A91")))}>
               { "SignIn" }
              </button>
              {value}
            </div>
           </main>
           }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
