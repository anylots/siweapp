import React from 'react'
import { Layout, Menu } from 'antd'
import { Route, Switch, Redirect, NavLink } from 'react-router-dom'
import DirectAward from './DirectAward'

const { Content, Sider } = Layout;

const Test = (props) => {
    return (
        <Layout className="sider_layout" style={{ minHeight: '100vh' }}>
            <Sider collapsed={false} >
                <div className="logo header_title">
                    项目管理
                </div>
                <Menu theme="dark" selectedKeys={[props.location.pathname]} mode="inline" >
                    <Menu.Item key="/home/users">
                        <NavLink to="/home/users">用户列表</NavLink>
                    </Menu.Item >
                    <Menu.Item key="/home/roles">
                        <NavLink to="/home/roles">角色列表</NavLink>
                    </Menu.Item >
                </Menu>
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
    )
}

export default Test