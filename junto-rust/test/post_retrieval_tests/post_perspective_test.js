const { Config, Container, Scenario } = require("@holochain/holochain-nodejs")
//const n3h = require('n3h');
Scenario.setTape(require('tape'))

const dnaPath = "./dist/junto-rust.dna.json"

const dna = Config.dna(dnaPath)
const agentEric = Config.agent("eric");
const agentJosh = Config.agent("josh")
const instanceJosh = Config.instance(agentJosh, dna)
const instanceEric = Config.instance(agentEric, dna);
const scenario = new Scenario([instanceJosh, instanceEric], {debugLog: true}) 

scenario.runTape('Can add, get users from perspective and get posts from a perspective', async (t, {josh, eric}) => {
    //create user josh
    const register_result = await josh.callSync('core', 'create_user', {user_data: {username: "jdeepee", first_name: "Josh", last_name: "Parkin", bio: "Junto Testing", profile_picture: "pictureurl"}});
    console.log("Register user result", register_result);
    t.equal(JSON.stringify(register_result), JSON.stringify({"Ok":{"private_den":{"address":"QmV7H3Mhpdpj9NfFq2pgwzRd83uEjQupsHa5zwVVeCWSd2","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmV9j9LNfc4spvT8qNA24vjMjC4JEnoVfidfiBfnY4PUs3","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"Qmc48qWCdrCEqJVn1a4XZd6Eyrsu1W5jHHi1CgsVJgEAMx","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"QmW8j2NrAvKzUTQxtYnPGXmz7PgRevrGgvkD21jHC2utHA","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Josh\'s Pack","owner":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","privacy":"Shared"}},"profile":{"address":"QmQ2UTpz5EGD3v5N5iZe6FwaGWgbFGazATTC2RQvB5SuR1","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","first_name":"Josh","last_name":"Parkin","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","entry":{"username":"jdeepee"}},"user_perspective":{"address":"QmcBgVN5mo8ACrX1Z1f2ZXNFzbRWSGhMskuNoJXe9fYQ71","entry":{"parent":"QmT7TDNsrKw2psyvYJztAMVFyKowPtR5VLbwDVHbtuoWSn","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}))
    console.log("Completed register profile\n\n\n")

    //create user eric
    const register_result_eric = await eric.callSync('core', 'create_user', {user_data: {username: "sunyatax", first_name: "Eric", last_name: "Yang", bio: "Junto Testing", profile_picture: "pictureurl"}});
    console.log("Register user eric result", register_result_eric);
    t.equal(JSON.stringify(register_result_eric), JSON.stringify({"Ok":{"private_den":{"address":"QmXi3gko95vvsYWhdTocjBPHTUZsBKnu9coZc4EJKMkwFe","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Private","channel_type":"Den"}},"shared_den":{"address":"QmTunjPizi21fTRU23WufNVMFAXLd1XRnYDab9nynrYgKD","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Shared","channel_type":"Den"}},"public_den":{"address":"QmUWkBB8ttdEUf6nEyMWodrJQcgv9fuQKFuRg7A6hKSj41","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Den","privacy":"Public","channel_type":"Den"}},"pack":{"address":"QmTevRrtjaaJzNCESubqfZfZjNXxJH4RxFBKFb9Nd7LUWh","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Eric\'s Pack","owner":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","privacy":"Shared"}},"profile":{"address":"QmXF2BASNKjFg76hth4b6PJ4Btj6oeJLmn7AuZerCLrZiM","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","first_name":"Eric","last_name":"Yang","bio":"Junto Testing","profile_picture":"pictureurl","verified":true}},"username":{"address":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","entry":{"username":"sunyatax"}},"user_perspective":{"address":"QmSxHAHwnE3Qw76uS6jviKMkVW1g3LCU828hxsqKqU6dpd","entry":{"parent":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","name":"Default Perspective","privacy":"Private","channel_type":"Perspective"}}}}));
    console.log("Completed register profile\n\n\n");

    //add user eric to josh's default perspective
    const add_user_to_perspective = await josh.callSync('core', 'add_user_to_perspective', {perspective: "QmcBgVN5mo8ACrX1Z1f2ZXNFzbRWSGhMskuNoJXe9fYQ71", target_user: 'QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU'});
    console.log("Add user to perspective result", add_user_to_perspective);
    t.equal(JSON.stringify(add_user_to_perspective), JSON.stringify({ Ok: 'QmZgTybnzjm9wr7GR2PGFYUpHESsVBiMAfiS6JjTNxBLFz' }));
    console.log('Completed add user to perspective');

    const perspective_users = await josh.callSync('core', 'get_perspectives_users', {perspective: "QmcBgVN5mo8ACrX1Z1f2ZXNFzbRWSGhMskuNoJXe9fYQ71"});
    console.log("User perspective results: ", perspective_users);
    t.equal(JSON.stringify(perspective_users), JSON.stringify({"Ok":[{"address":"QmYCk7czLzXxbvCucMA8HTxyVbHtKz95egfkYhBhznmZcU","entry":{"username":"sunyatax"}}]}));
    console.log('Completed user perspective results');

    const holochain_env = await eric.callSync('core', 'show_env', {});
    console.log("Show env result, holochain_env", holochain_env);
    const dna = holochain_env.Ok.dna_address;
    console.log("DNA of application: ", dna, "\n\n\n");

    //Post expression to one context (global) with all four tags specified - all unique with one tag having an uppercase letter
    const post_global_expression = await eric.callSync('core', 'post_expression', {
        expression: 
                {
                    expression: {
                        PostExpression: {
                            post: "This is the first test expression"
                        }
                    },
                    expression_type: "PostExpression"
                }, 
        tags: ["holochain", "Junto", "social", "holo"], 
        context: [dna]
    });
    console.log("Post expression 1 result", post_global_expression);
    t.equal(JSON.stringify(post_global_expression), JSON.stringify({ Ok: 'QmZ23wNYx8BNtHMcG6kYNufycR6s8dXyqWP6ySYsTbHnPg' }));
    console.log("Completed posting expression\n\n\n\n");

    //Make query for post on the created perspective
    let d = new Date();
    let year = d.getFullYear();
    let month = d.getUTCMonth() + 1;
    let day = d.getUTCDate();
    let hour = d.getUTCHours();    
    const perspective_query = await josh.callSync('core', 'get_expression', {perspective: "QmcBgVN5mo8ACrX1Z1f2ZXNFzbRWSGhMskuNoJXe9fYQ71", 
                                                                            query_points: ["social<tag>", "junto<tag>", "holochain<tag>", "holo<tag>", "sunyatax<user>", "postexpression<type>", year+"<time:y>", "0"+month+"<time:m>", day+"<time:d>", hour+"<time:h>"],
                                                                            query_options: "FilterNew",
                                                                            target_type: "ExpressionPost",
                                                                            query_type: "And",
                                                                            dos: 1,
                                                                            seed: "totally random seed"});
    console.log("Make 1 dos query result", perspective_query);
    t.equal(JSON.stringify(perspective_query), JSON.stringify({"Ok":[{"address":"QmZ23wNYx8BNtHMcG6kYNufycR6s8dXyqWP6ySYsTbHnPg","entry":{"expression_type":"PostExpression","expression":{"PostExpression":{"post":"This is the first test expression"}}}}]}));
    console.log("Completed\n")
})