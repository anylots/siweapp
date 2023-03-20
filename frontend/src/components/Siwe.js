import { ethers } from 'ethers';
import { SiweMessage } from 'siwe';
import React, { Component } from 'react'
import { Button, Input } from 'antd';
const { TextArea } = Input;

const domain = window.location.host;
const origin = window.location.origin;
const provider = new ethers.providers.Web3Provider(window.ethereum);
const signer = provider.getSigner();



export default class Siwe extends Component {

    constructor(props) {
        super(props);

        // We store multiple things in Dapp's state.
        // You don't need to follow this pattern, but it's an useful example.
        this.initialState = {
            // The info of the token (i.e. It's Name and symbol)
            tokenData: undefined,
            // The user's address and balance
            selectedAddress: undefined,
            balance: undefined,
            // The ID about transactions being sent, and any possible error with them
            txBeingSent: undefined,
            transactionError: undefined,
            networkError: undefined,
        };

        this.state = this.initialState;
    }

    render() {
        return (

            <div>
                <div style={{ marginLeft: "220px", "fontSize": "18px" }}>
                    {/* Accounts you might be interested in */}
                    <div class="alert alert-info" role="alert" style={{ width: "600px" }}>
                    </div>
                </div>
                <Button type="primary" style={{ marginLeft: "120px", marginTop: "20px" }} >DepositEther</Button>


                <div style={{ marginTop: "30px", display: "flex", justifyContent: "space-between", marginLeft: "120px" }}>
                    <div class="form-group">
                        <Button type="primary" style={{ marginTop: "20px" }} onClick={() => this.signInWithEthereum()}>SignInWithEthereum</Button>
                    </div>
                </div>
            </div>

        )

    }


    createSiweMessage(address, statement) {
        const message = new SiweMessage({
            domain,
            address,
            statement,
            uri: origin,
            version: '1',
            chainId: '1'
        });
        return message.prepareMessage();
    }

    connectWallet() {
        provider.send('eth_requestAccounts', [])
            .catch(() => console.log('user rejected request'));
    }

    async signInWithEthereum() {
        const message = this.createSiweMessage(
            await signer.getAddress(),
            'Sign in with Ethereum to the app.'
        );

        let sig = await signer.signMessage(message);
        console.log(sig);

        let param = { message: message, sig: sig };


        await fetch('http://127.0.0.1:3030/sign_in', {
            method: 'post',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(param)

        }
        ).then(res => {
            console.log(res);
        })

    }
}


