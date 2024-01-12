use auth::configuration::get_configuration;
use auth::proto::auth::auth_client::AuthClient;
use auth::proto::auth::LoginRequest;
use tonic::Request;
use tonic_types::StatusExt;

use std::error::Error;
// bring in our client

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // the tonic crate uses the transport module to crate a channel which actually connect to the service and allows us to call the,
    // here it does not include the rpc that we have defined this is what tonic build actually does it create a client for us which is essentially a wrapper
    // it internally uses channel and will use the call method that is defined on channel that takes in our request. but the client that is generated has function
    // for our rpc with definitions on the input types.
    //
    // it also includes the with_interceptor functions which is helpful since it sorts like a middleware but not as flexible here one changes the data that is defined
    // in the metadata map,
    //
    // finally tonic has a EndPoint which is essentially a channel builder which is actually what is used to create the channel so one provides the uri, extra configuration
    // that has to be done then one can use the connect methods provided to finialize and crate the channel.
    //
    // one can skip configuring a channel and create the client directly using the client connect with works fine but it all depends on what you want to do & achieve for
    // http/2 support multiplexing ie sending multiple request over the same connection and this can be used to send request two 2 services using the same connection
    // look at https://github.com/hyperium/tonic/tree/master/examples/src/multiplex for more details

    let configuration = get_configuration().expect("Failed to get configuration");
    let address = format!("http://[::1]:{}", configuration.application_port);

    let mut client = AuthClient::connect(address).await?;

    let response = client
        .login(Request::new(LoginRequest {
            username: "atheer2104".into(),
            password: "".into(),
        }))
        .await;

    match response {
        Ok(res) => {
            println!("RESPONSE={:?}", res);
        }
        Err(status) => {
            // note that these are just bytes
            let error_details = status.get_error_details();

            // check if we have a bad request
            if let Some(bad_request) = error_details.bad_request() {
                println!("BAD_REQUEST: {:?}", bad_request)
            }

            if let Some(message) = error_details.localized_message() {
                println!("localized message: {:?}", message);
            }
        }
    }

    Ok(())
}
