import React, { Component } from 'react'

// We'll use ethers to interact with the Ethereum network and our contract
import SignIn from "./SignIn";


export default class Identifier extends Component {
    constructor(props) {
        super(props);
    }

    render() {

        return (
            <div style={{ "text-align":"center"}}>

                <div calssName="card" style={{ height: "440px", "border-radius": "0.25rem", background: "linear-gradient(44.19deg,#1c7bff -.88%,#9d6fff 97.74%)" }}>
                    {/* <img src={passport} style={{ marginLeft: "-24px" }} class="card-top" /> */}
                    <div style={{ "backgroundColor": "#f8f9fa33", "top": "0px", "width": "100%" }}>
                        <h1 style={{
                            color: "#ffffff", "WebkitBackgroundClip": "text", "opacity": 0.9
                            , "fontSize": "8px", "line-height": "24px"
                        }}> R O P S T E N
                        </h1>
                    </div>

                    <div><img src={this.props.avatar} width="40%" style={{ "border-radius": "100px" }} /></div>
                    <div>
                        <span class="badge badge-dark">{this.props.nickName}</span>
                    </div>

                    <div style={{ display: "flex", padding: "15px 15px" }}>
                        <span class="badge badge-dark" style={{  }}>Region:</span>
                        <span class="badge badge-dark" style={{ marginLeft: "5px" }}>{this.props.region}</span>
                        <span class="badge badge-dark" style={{ marginLeft: "5px" }}>Ethereum</span>
                        {/* <span class="badge badge-dark" style={{ marginLeft: "5px" }}>Sol</span>
                        <span class="badge badge-dark" style={{ marginLeft: "5px" }}>Near</span> */}
                    </div>

                    <div>
                        <span class="badge badge-light" style={{}}>Current In Ethereum</span>
                    </div>

                    <div style={{ marginTop: "40px" }}>
                        <SignIn action="Sign-In With Ethereum" />
                    </div>
                </div>
                <div>
                </div>
            </div>

        )
    }
}
