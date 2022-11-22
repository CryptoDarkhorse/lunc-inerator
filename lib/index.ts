import type { Env } from "@terra-money/terrain";
import { LuncIneratorClient } from './clients/LuncIneratorClient';

export class Lib extends LuncIneratorClient {
  env: Env;

  constructor(env: Env) {
    super(env.client, env.defaultWallet, env.refs['lunc-inerator'].contractAddresses.default);
    this.env = env;
  }
};

export default Lib;
