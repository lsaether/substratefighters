import React from 'react';
import { ReactiveComponent, Rspan, If } from 'oo7-react';
const { Pretty } = require('../Pretty');
import { Card } from 'semantic-ui-react'
import { runtime, secretStore } from 'oo7-substrate';
import Identicon from 'polkadot-identicon';
import { KittyAvatar } from './avatars';
import './KittyCards.css'

class KittyCard extends ReactiveComponent {
    constructor(props) {
        super(['kitty', 'owner'])
    }

    readyRender() {
        let kitty = this.state.kitty;

        if (kitty != null) {
            return <Card>
                <KittyAvatar dna={kitty.dna} />
                <Card.Content>
                    <Card.Header><Pretty value={kitty.id} className="limit-name" /></Card.Header>
                    <Card.Meta>
                        <Pretty value={kitty.dna} className="limit-name" />
                    </Card.Meta>
                    <Rspan>
                        <b>Owner</b>: {secretStore().find(this.state.owner).name}
                    </Rspan>
                    &nbsp;
                            <Identicon key={this.state.owner} account={this.state.owner} size={16} />
                    <br />
                    <Rspan>
                        <b>Generation</b>: {kitty.gen}
                    </Rspan>
                    <br />
                    <If condition={kitty.speed != 0 && kitty.speed != null} then={<div>
                    <Rspan>
                        <b>Speed</b>: {kitty.speed}
                    </Rspan>
                    <br />
                    </div>}/>
                </Card.Content>
                <Card.Content extra>
                    <Pretty value={kitty.price} prefix="$" />
                </Card.Content>
            </Card>;
        } else {
            return <span>Upgrade your Substrate Kitties UI for the latest version.</span>
        }
    }
}

class KittyWrap extends ReactiveComponent {
    constructor(props) {
        super(['hash'])
    }

    readyRender() {
        // one level of indirection: convert a given hash
        // to the request of the actual kitty data and who it belongs to
        return <KittyCard
            kitty={eval('runtime.substratekitties.kittiesV' + window.substrateKittiesVersion + '(this.state.hash)')}
            owner={runtime.substratekitties.kittyOwner(this.state.hash)}
        />
    }
}
export class KittyCards extends ReactiveComponent {
    constructor(props) {
        super(['count'])
    }
    unreadyRender() {
        return <span>No kittens found yet</span>
    }
    readyRender() {
        let kitties = [];
        for (var i = 0; i < this.state.count; i++) {
            kitties.push(
                <div className="column" key={i}>
                    <KittyWrap hash={runtime.substratekitties.allKittiesArray(i)} />
                </div>
            );
        }

        return <div className="ui stackable six column grid">{kitties}</div>;
    }
}
