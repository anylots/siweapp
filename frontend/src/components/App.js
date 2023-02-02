import React, { Component } from 'react';
import { Route, Switch, HashRouter, NavLink, Link } from 'react-router-dom';
import DirectAward from './DirectAward';

import Identifier from './Identifier';
import ConnectWallet from './ConnectWallet';
import avatar from "../images/show/avatar/avatar1.jpg";
import { Layout, Menu } from 'antd'

const { Content, Sider } = Layout;
const HARDHAT_NETWORK_ID = '1337';


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
                                , "fontFamily": "Galaxy", "fontSize": "20px", "marginTop": "10px", "marginLeft": "20px"
                            }}>Joyboy
                            </h1>
                        </div>
                        <div className="collapse navbar-collapse" id="navbarText" style={{ color: "#ffffff" }}>
                        </div>
                    </nav>
                    <div style={{ float: "right", width: "620px" }}>
                        <a className="navbar-brand" href="#" style={{
                            position: "absolute", "-webkit-font-smoothing": "antialiased", fontFamily: "Montserrat",
                            top: "13px", "fontSize": "16px", color: "#ffffffbb"
                        }}>Ethereum Marketing operation workbench
                        </a>

                        {/* connect to web3 */}
                        <div style={{ marginLeft: "320px" }} >
                            <ConnectWallet />
                        </div>

                    </div>
                </div>

                <div style={{ marginTop: "40px" }}>
                    <div >
                        <HashRouter>
                            <Layout  style={{ minHeight: '100vh', display: "flex" }}>
                                <Sider collapsed={false} >
        
                                    <Menu theme="dark" mode="inline" >
                                        <Menu.Item key="/home/users">
                                            <NavLink to="/home/users">用户列表</NavLink>
                                        </Menu.Item >
                                        <Menu.Item key="/home/roles">
                                            <NavLink to="/home/roles">角色列表</NavLink>
                                        </Menu.Item >
                                    </Menu>
                                    <div style={{ marginTop: "60px" }}>
                                        <Identifier region="Avalanche" avatar={avatar} nickName="Test"></Identifier>
                                    </div>
                                </Sider>
                                <Layout>
                                    <Content className="us_content">
                                        <Switch>
                                            <Route path="/home/users" component={DirectAward} />
                                            <Route path="/home/roles" component={DirectAward} />
                                        </Switch>

                                    </Content>
                                </Layout>
                            </Layout>
                        </HashRouter>

                    </div>
                </div>
            </div>
        );
    }

}
