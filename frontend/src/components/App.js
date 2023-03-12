import React from 'react';
import Siwe from './Siwe';
import Identifier from './Identifier';
import avatar from "../images/show/avatar/avatar1.jpg";

export default class App extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {
        return (
            <div >
                {/* top navbar */}
                <div>
                    <nav className="navbar navbar-expand-lg " style={{ "backgroundColor": "#000000", "opacity": 0.4, "top": "0px", "width": "100%" }}>
                        <div>
                            <h1 style={{
                                color: "#FFFFFF", "WebkitBackgroundClip": "text"
                                , "fontFamily": "Galaxy", "fontSize": "20px", "marginTop": "0px", "marginLeft": "20px"
                            }}>SIWE APP
                            </h1>
                        </div>
                        <div className="collapse navbar-collapse" id="navbarText" style={{ color: "#ffffff" }}>
                        </div>
                    </nav>
                    <div style={{ float: "right", width: "620px" }}>
                        <a className="navbar-brand" href="#" style={{
                            position: "absolute", "-webkit-font-smoothing": "antialiased", fontFamily: "Montserrat",
                            top: "3px", "fontSize": "16px", color: "#ffffffbb", "marginLeft": "280px"
                        }}>Sign With In Ethereum
                        </a>
                    </div>
                </div>

                <div style={{ marginTop: "-10px", width: "240px", display: "flex" }}>
                    <div style={{ marginTop: "60px", width: "240px" }}>
                        <Identifier region="Avalanche" avatar={avatar} nickName="Test"></Identifier>
                    </div>
                    <div style={{ marginTop: "60px", width: "240px" }}>
                        <Siwe region="Avalanche" avatar={avatar} nickName="Test"></Siwe>
                    </div>

                </div>
            </div>
        );
    }

}
