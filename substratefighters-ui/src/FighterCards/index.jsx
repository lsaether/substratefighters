import React from 'react';
import { ReactiveComponent, Rspan, If } from 'oo7-react';
const { Pretty } = require('../Pretty');
import { Card } from 'semantic-ui-react'
import { runtime, secretStore } from 'oo7-substrate';
import Identicon from 'polkadot-identicon';
import { Avatar } from './img';
import './FighterCards.css'

class FighterCard extends ReactiveComponent {
    constructor(props) {
        super(['fighter', 'owner'])
    }

    readyRender() {
        let fighter = this.state.fighter;

        if (fighter != null) {
            return <Card>
                <Avatar dna={fighter.dna} />
                <Card.Content>
                    <Card.Header><Pretty value={fighter.id} className="limit-name" /></Card.Header>
                    <Card.Meta>
                        <Pretty value={fighter.dna} className="limit-name" />
                    </Card.Meta>
                    <Rspan>
                        <b>Owner</b>: {secretStore().find(this.state.owner).name}
                    </Rspan>
                    &nbsp;
                            <Identicon key={this.state.owner} account={this.state.owner} size={16} />
                    <br />
                    <Rspan>
                        <b>Strength</b>: {fighter.strength}
                    </Rspan>
                    <br />
                    <If condition={fighter.wins != null} then={<div>
                    <Rspan>
                        <b>Wins</b>: {fighter.wins}
                    </Rspan>
                    <br />
                    </div>}/>
                </Card.Content>
                <Card.Content extra>
                    <Pretty value={fighter.price} prefix="$" />
                </Card.Content>
            </Card>;
        } else {
            return <span>Upgrade your Substrate Kitties UI for the latest version.</span>
        }
    }
}

class FighterWrap extends ReactiveComponent {
    constructor(props) {
        super(['hash'])
    }

    readyRender() {
        // one level of indirection: convert a given hash
        // to the request of the actual kitty data and who it belongs to
        return <FighterCard
            fighter={eval('runtime.substratefighters.fightersV' + window.substrateFightersVersion + '(this.state.hash)')}
            owner={runtime.substratefighters.fighterOwner(this.state.hash)}
        />
    }
}
export class FighterCards extends ReactiveComponent {
    constructor(props) {
        super(['count'])
    }
    unreadyRender() {
        return <span>No fighters found yet</span>
    }
    readyRender() {
        let fighters = [];
        for (var i = 0; i < this.state.count; i++) {
            fighters.push(
                <div className="column" key={i}>
                    <FighterWrap hash={runtime.substratefighters.allFightersArray(i)} />
                </div>
            );
        }

        return <div className="ui stackable six column grid">{fighters}</div>;
    }
}
