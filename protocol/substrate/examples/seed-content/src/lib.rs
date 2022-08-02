use wasm_bindgen::prelude::*;
use forum_app::Api;
use codec::Decode;
use serde_json::json;


async fn seed1(api: &Api) -> anyhow::Result<()> {
    let entries: Vec<(&str,Vec<(&str, Vec<&str>)>)> = vec![
        ("This is content1",
            vec![
                ("This is comment1 of content1",vec![]),
                ("This is comment2 of content1",vec!["This is reply of comment2 of content1"]),
                ("This is comment3 of content1",vec![]),
            ]
        ),

        ("I’d just like to interject for a moment.\
         \nWhat you’re refering to as Linux, is in fact, GNU/Linux, or as I’ve recently taken to calling it, GNU plus Linux. ",
        vec![
            ("Linux is not an operating system unto itself,\
             \nbut rather another free component of a fully functioning GNU system made useful by the GNU corelibs,\
             \nshell utilities and vital system components comprising a full OS as defined by POSIX.",
             vec![]
            ),
            ("Many computer users run a modified version of the GNU system every day, without realizing it. ",
             vec![]
            ),
            ("Through a peculiar turn of events, the version of GNU which is widely used today is often called Linux,\
             and many of its users are not aware that it is basically the GNU system, developed by the GNU Project.",
             vec![]
            ),
            ("There really is a Linux, and these people are using it, but it is just a part of the system they use.\
             \nLinux is the kernel: the program in the system that allocates the machine’s resources to the other programs that you run.",
             vec![]
             ),
             ("The kernel is an essential part of an operating system,\
              \nbut useless by itself; it can only function in the context of a complete operating system.",
              vec![]
             ),
             ("Linux is normally used in combination with the GNU operating system:\
              \nthe whole system is basically GNU with Linux added, or GNU/Linux.",
             vec![]
             ),
             ("All the so-called Linux distributions are really distributions of GNU/Linux!",
             vec![]
            ),
        ]),

        ("Thou TCP/IP ensures the delivery and acknowledge,\
        \nbut UDP sacrifice accuracy for speed for applications such as games and movies, users don't want to wait\
        \n-- Sun Tzu, 1337 AD",
        vec![
        ]),

        ("Shakespeare quote of the Day:\
        \nAn SSL error has occured and a secure connection to the server cannot be made.",
         vec![
         ("Bruh",vec![]),
         ]
        ),

        ("His palms are sweaty\
        \nKnees weak, arms are heavy\
        \nThe unit tests are failing already\
        \nCode spaghetti",
        vec![
        ("He's nervous,\
            \nBut at his laptop he looks calm and ready\
            \nTo squash bugs\
            \nBut he keeps on forgetting",
            vec![]),

            ("What he typed out\
            \nThe key taps grow so loud\
            \nHe checks his commits\
            \nBut the logs won’t turn out\
            \nHe’s spacing, how\
            \nEverybody’s pacing now\
            \nThe clock’s run out, deadline\
            \nIt’s due now!",
            vec![]
            ),


            ("Snap back to the IDE,\
            \nOh, there goes TDD\
            \nOh there goes habits he knows\
            \nHe’s so mad but he goes\
            \nDeeper in debt that easy\
            \nNo, he won’t have it\
            \nHe knows, his old build server\
            \nWoke, he knows his whole build will be broke\
            \nIt don’t matter, he’ll cope",
            vec![]
            ),
            ]
        ),
    ];


    for (post, replies0) in entries {
        println!("post: {}", post);
        let post_id = add_post(&api, post).await?;
        for (reply, replies1) in replies0 {
            println!("\t>{}", reply);
            let comment_id = add_comment(&api, post_id, reply).await?;
            for reply in replies1 {
                println!("\t\t>{}", reply);
                let _comment_id =
                    add_comment(&api, comment_id, reply).await?;
            }
        }
    }
    more_seed(&api).await?;
    Ok(())
}

async fn more_seed(
    api: &Api,
) -> anyhow::Result<()> {
    let chain = vec![
        "Gordon Ramsay doesn't like being called \"mate\"",
        "I'm not your mate buddy",
        "I'm not your buddy, pal",
        "I'm not your pal, friend",
        "I'm not your friend, cuz",
        "I'm not your cuz, bro",
        "I'm not your bro, mate",
        "I'm not your mate, dog",
        "I'm not your dog, dude",
        "I'm not your dude, broski",
        "I'm not your broski, son",
        "I'm not your son, dad",
        "I'm not your dad, son",
        "I'm not your son, acquaintances of mine",
        "I'm not your acquaintances, love",
        "I'm not your love, sweetheart",
        "I'm not your sweetheart, babe",
        "I'm not your babe, darling",
        "I'm not your darling, dearie",
        "I'm not your dearie, honey",
        "I'm not your honey, sugar",
        "I'm not your sugar, baby",
        "I'm not your baby, sweetie",
        "I'm not your sweetie, lover",
        "I'm not your lover, precious",
        "That's it, that's enough internet for me today",
        "I'm not your internet, random dude",
        "I'm not your random dude, Dad",
    ];

    let mut parent_item = add_post(api, chain[0]).await?;
    println!("post: {}", chain[0]);

    for (i, reply) in chain.iter().skip(1).enumerate() {
        println!("reply: {}", reply);
        parent_item = add_comment(api, parent_item, reply).await?;
    }
    Ok(())
}

async fn add_comment(api:&Api, parent_item: u32, reply: &str) -> anyhow::Result<u32>{
    let current_item = get_current_item(api).await?;
    forum_app::fetch::add_comment(api, parent_item, reply).await?;
    Ok(current_item)
}

async fn add_post(api:&Api, post: &str) -> anyhow::Result<u32>{
    let current_item = get_current_item(api).await?;
    forum_app::fetch::add_post(api, post).await?;
    Ok(current_item)
}



/// Note: this is a very unreliable way of determining
/// the post_id, comment_id of the content that was just posted
/// since some other threads could be using them.
/// This is for the sake of simulating replying to some post the user have seen on the UI.
async fn get_current_item(api: &Api) -> anyhow::Result<u32> {
    let args = json!({
        "url": api.url,
        "pallet": "ForumModule",
        "storage": "ItemCounter",
    });

    let current_item: Option<Vec<u8>> =
        api.invoke_method("getStorageValue", args).await?;

    if let Some(current_item) = current_item {
        let current_item: u32 = Decode::decode(&mut current_item.as_slice()).expect("must decode");
        log::info!("current item: {}", current_item);
        Ok(current_item)
    } else {
        log::error!("can not get the ItemCounter, likely an error");
        Ok(0)
    }
}

#[wasm_bindgen]
pub async fn start_seed() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();
    log::info!("Starting to put seed content into forum");
    let api = Api::new("http://localhost:9933").await.expect("must not error");
    add_post(&api, "Posted from seeding app").await.expect("must not error");
    seed1(&api).await.expect("must have no error");

}
