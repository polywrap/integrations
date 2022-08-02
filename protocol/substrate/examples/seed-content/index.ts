import { PolywrapClient } from "@polywrap/client-js";
import { ipfsPlugin } from "@polywrap/ipfs-plugin-js";
import { ethereumPlugin } from "@polywrap/ethereum-plugin-js";
import { ensResolverPlugin } from "@polywrap/ens-resolver-plugin-js";


let module_uri = "wrap://ipfs/QmQDqV2Y3QD87CG48quXmWXigLBZ3J2qn5z4BY4ZhwMTtL"

class PolywrapClientWrapper extends PolywrapClient{
    constructor() {
        super({
            plugins: [
                {
                  uri: "wrap://ens/ipfs.polywrap.eth",
                  plugin: ipfsPlugin({
                      provider: "https://ipfs.wrappers.io"
                  }),
                }
            ]
        })
    }

    async invoke_method(method, args){
        console.log("invoking method:", method);
        console.log("with args:", args);

        let result = await super.invoke({
            uri: module_uri,
            method,
            args,
        });

        console.log("got result: ", result);

        return result.data;
    }
}

// attach the PolywrapClient to the window so we can access it later in the rust code
// See `src/api.rs` this is linked into the rust code using `wasm-bindgen`.
window.PolywrapClientWrapper = PolywrapClientWrapper;
