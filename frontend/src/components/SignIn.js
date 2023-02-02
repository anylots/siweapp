import React, { Component } from 'react'

import { NetworkErrorMessage } from "./NetworkErrorMessage";

const HARDHAT_NETWORK_ID = '1337';
const GOERLI = '5';


export default class SignIn extends Component {

  constructor(props) {
    super(props);
    this.initialState = {

      // The info of the token (i.e. It's Name and symbol)
      tokenData: { name: "NFT", symbol: "NFT" },
      // The user's address and balance
      selectedAddress: undefined,
      balance: 10000,
      // The ID about transactions being sent, and any possible error with them
      txBeingSent: undefined,
      transactionError: undefined,
      networkError: undefined,
      buttonText: "Connect"
    };

    this.state = this.initialState;
  }
  render() {
    return (
      <div className="container">
        <div className="row justify-content-md-center">
          <div className="col-12 text-center">
            {/* Metamask network should be set to Localhost:8545. */}
            {this.state.networkError && (
              <NetworkErrorMessage
                message={this.state.networkError}
                dismiss={this.dismissNetworkError()}
              />
            )}
          </div>
          <div className="p-3 text-center">
            {/* <p>Please connect to your wallet.</p> */}
            <button
              className="btn btn-light btn-sm"
              type="button"
              onClick={() => this.connectWallet()}
            >
              <span style={{ whiteSpace: "nowrap" }}>{this.props.action}</span>
            </button>
          </div>
        </div>
      </div>
    );
  }


  /**
   * @description: connectWallet
   * @param {*}
   * @return {*}
   */
  async connectWallet() {
    // This method is run when the user clicks the Connect. It connects the
    // dapp to the user's wallet, and initializes it.


    //check metamsk is installed
    if (!this.installedMetaMask()) {
      window.open("https://metamask.io/", "install metamsk");
    }

    // To connect to the user's wallet, we have to run this method.
    // It returns a promise that will resolve to the user's address.
    const [selectedAddress] = await window.ethereum.request({ method: 'eth_requestAccounts' });

    // Once we have the address, we can initialize the application.

    // First we check the network
    if (!this.checkNetwork()) {
      await this.switchToEthereum();
      return;
    }
    console.log(selectedAddress);
    this.setState({ selectedAddress: selectedAddress, buttonText: selectedAddress.substring(0, 9) });


    // We reinitialize it whenever the user changes their account.
    window.ethereum.on("accountsChanged", ([newAddress]) => {
      // `accountsChanged` event can be triggered with an undefined newAddress.
      // This happens when the user removes the Dapp from the "Connected
      // list of sites allowed access to your addresses" (Metamask > Settings > Connections)
      // To avoid errors, we reset the dapp state 
      this.setState({ selectedAddress: newAddress, buttonText: newAddress.substring(0, 9) });
    });

    // We reset the dapp state if the network is changed
    window.ethereum.on("chainChanged", ([networkId]) => {
      this.resetState();
    });
  }

  // This method just clears part of the state.
  dismissNetworkError() {
    this.setState({ networkError: undefined });
  }

  // This method checks if Metamask selected network is Localhost:8545 
  checkNetwork() {
    if (window.ethereum.networkVersion === GOERLI) {
      return true;
    }

    this.setState({
      networkError: 'Please connect Metamask to Goerli'
    });

    return false;
  }

  /**
 * @description: check metamsk
 * @param {*}
 * @return {bool}
 */
  installedMetaMask() {
    const { ethereum } = window;
    return Boolean(ethereum && ethereum.isMetaMask);
  }


  /**
   * @description: Switch to the Goerli test network (may be deprecated in the future)
   * @param {*}
   * @return {*}
   */
  async switchToEthereum() {
    try {
      await window.ethereum.request({
        method: "wallet_switchEthereumChain",
        params: [
          {
            chainId: "0x5"
          }
        ]
      });
    } catch (error) {
      console.log(error);
    }
  }

  //reset the dapp state
  resetState() {
    this.setState(this.initialState);
  }

}

