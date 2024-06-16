use std::env;

use models::{gpt_transcribe::TranscribeSegment, news_article::NewsArticle};
use once_cell::sync::OnceCell;
use services::{db_helper::DbService, ffmpeg_helper, local_envs::Config, utils};

mod models;
mod services;

fn test_scraper() {
    let article_url = "https://t.co/jpC8waRyKi";
    let news_article = services::news_crawler::scrape_cnn_news(&article_url);

    match news_article {
        Some(article) => println!("{}", serde_json::to_string(&article).unwrap()),
        None => {}
    }
}

fn test_summary() {
    let response = services::chat_gpt_helpers::get_summary("Boeing’s Starliner mission has safely docked with the International Space Station and the spacecraft’s crew, NASA astronauts Butch Wilmore and Suni Williams, have arrived aboard the station after overcoming new issues that cropped up overnight and Thursday en route to the orbiting laboratory. This is the first time astronauts have arrived at the space station via a Boeing Starliner spacecraft. Docking occurred at 1:34 p.m. ET. Steps were taken to more firmly secure the connection between Starliner and the space station’s port, and docking was completed about 20 minutes later. “Nice to be attached to the big city in the sky,” Wilmore said after docking was confirmed. Pressure was equalized between Starliner and the station, and the hatch between the two opened at about 3:46 p.m. ET. Wilmore and Williams were welcomed with a ringing bell and plenty of hugs from the seven astronauts and cosmonauts already on board. With their arrival, there are now nine people working and living on the ISS. “I’m not sure we could have gotten a better welcome,” Wilmore said during welcoming remarks after the entire station crew assembled. “We had music. Matt was dancing. It was great. What a wonderful place to be back.” Wilmore shared his thanks “to all of you who got us going and kept us going,” including those in NASA mission control, Boeing and United Launch Alliance. “These organizations are the epitome of teamwork, and it’s a blessing and a privilege to be a part of it,” he said. Williams also shared her gratitude to the family and friends who have been with them during the lead-up to launch. “We have another family up here, which is just awesome,” Williams said. “And we’re just happy as can be to be up in space, one in Starliner on an Atlas V, and then here at the International Space Station. It just doesn’t get much better.” “The launch yesterday and docking today puts the Starliner on a path to certification to enabling continued exploration and science that benefits humanity,” said Jim Free, NASA Associate Administrator, during a news conference Thursday. “For Butch and Suni, I want to thank them for the years they have spent to getting us to this point, the expertise they bring to their roles and their dedication to the task of advancing human spaceflight,” he added. The duo will spend the next eight days on the orbiting laboratory. The crew brought a much-needed replacement pump that processes urine aboard the station and turns it into drinking water. The station crew will try to put the new pump into place tomorrow, said Jeff Arend, manager for systems engineering and integration in NASA’s International Space Station Office. Delayed arrival After a successful launch&nbsp;that was a decade in the making, Boeing’s Starliner mission experienced helium leaks and a temporary loss of thrusters during its journey to the space station, according to NASA. During the final hour of their approach to the space station, Starliner’s crew began to manually pilot the spacecraft in a planned test of Starliner’s manual flight control capability. Five of the reaction control system thrusters failed on the service module, but the duo was able to get four of the thrusters firing again after conducting hot-fire tests. These smaller thrusters are used as the spacecraft moves closer to the space station so it can make more finely tuned changes to its trajectory. There are 28 total such thrusters located on the service module, or the lower portion of Starliner, which will not return to Earth. Starliner was expected to dock at the space station by 12:15 p.m. ET, but the thruster issue caused a delay of an hour and 15 minutes, and the mission moved on to a new docking window. During testing of the thrusters, Starliner maintained a distance of about 820 feet (250 meters) from the space station until it was deemed “orbit-safe,” away from a 656-foot (200-meter) invisible “keep out” boundary designated to protect the station.  “We don’t necessarily understand exactly why we lost the jets,” said Steve Stich, manager for NASA’s Commercial Crew Program. “The software sees something that it doesn’t like about that thruster — maybe a little less thrust or the thrust rise rate doesn’t come up exactly the way that the software is looking for. And so what the software does, it says, ‘I won’t use that thruster anymore.’” The team will continue to evaluate the data behind the software that deselected the thrusters when they didn’t meet certain conditions set by the software, said Mark Nappi, vice president and program manager of the Commercial Crew Program for Boeing. Starliner also faced another issue on its journey: helium leaks. The space agency said late Wednesday in a post on X, formerly known as Twitter, that two additional helium leaks had been detected on the vehicle. One helium leak had been discovered prior to launch and deemed acceptable. “Helium is used in spacecraft thruster systems to allow the thrusters to fire and is not combustible or toxic,” according to Boeing. As of Thursday morning, two of the three leaks have been corrected, according to a live NASA broadcast. A fourth minor leak was discovered later as well, Stich said. Mission managers polled “go” for rendezvous and docking with the space station, and the leaks were not expected to impact docking, according to the broadcast. “During all of Starliner’s rendezvous and proximity operations, we’ll keep those propellant manifolds open, but they’ll stay open until docking. Starliner’s currently maintaining plenty of helium reserves,” Boeing aerospace engineer Jim May confirmed Thursday morning in a social media post on X shared by Boeing. “Currently the helium leak is not a safety issue for the crew, the vehicle or the mission.” The flight control team will continue to monitor the leak rates in Starliner’s propulsion system and after docking, all of Starliner manifolds were closed per normal plans and currently, there are no active leaks, according to NASA. “What we need to do over the next few days is take a look at the leak rate there and figure out what we go do relative to the rest of the mission,” he said. “I think we have some tools in our toolkit to manage this. We’ll take a little time to go, assess it and we’ll undock and land when we’re ready.” There may be some commonality between the leaks, Nappi said. “Now that we’re in flight and we’ve seen a couple more leaks, if it is a common cause across those flanges, then there might be something more to the flange itself, maybe a bad lot of seals or some other variable,” he said. Stich said the problems Starliner is facing is not unlike the first crewed flight of NASA’s Space Shuttle Program, or other test flights of spacecraft rated to take humans to space. “We have two problems on this vehicle: The helium leak and figuring out how to fine tune these thrusters so that they’re not selected off,” Nappi said. “Those are pretty small issues to deal with and we’ll figure them out for the next mission. So I don’t see these as significant at all.” Late night leaks Just as Wilmore and Williams were about to go to sleep Wednesday night, mission control informed them that they needed to shut down two valves due to the new helium leaks. “Teams have identified three helium leaks on the spacecraft. One of these was previously discussed before flight along with a management plan,” NASA shared in the post. “The other two are new since the spacecraft arrived on orbit. Two of the affected helium valves have been closed and the spacecraft remains stable.” A related exchange had taken place earlier on the NASA broadcast. “Looks like we picked up a couple more helium leaks,” mission control told the astronauts, as heard on the broadcast. Controllers then walked the crew through the plan to shut down the valves. “Butch, I’m sorry. We’re still getting the story together,” mission control replied. “We are ready to … find out exactly what you mean by picked up another helium leak, so give it to us,” Wilmore told them. NASA and Boeing determined the crew was safe and told the duo to go to sleep while they continue to look at the data. The crew was supposed to sleep for nine hours, but the troubleshooting effort cut into an hour of rest time. “We have some issues to watch overnight when in regards to the helium leaks that was just brought up, and we have a lot of smart people down here on the ground that are going to take a look at this stuff and keep an eye on it, but the vehicle is in a configuration right now where they’re safe to fly,” Boeing aerospace engineer Brandon Burroughs said on the NASA broadcast. Historic launch Starliner’s highly anticipated voyage lifted off atop an Atlas V rocket Wednesday at 10:52 a.m. ET from Cape Canaveral Space Force Station in Florida. The historic launch marked the first time the spacecraft has carried a crew to space. The mission, known as the Crew Flight Test, is the culmination of Boeing’s efforts to develop a spacecraft to rival&nbsp;SpaceX’s Crew Dragon capsule&nbsp;and expand the United States’ options for ferrying astronauts to the space station under NASA’s Commercial Crew Program. The federal agency’s initiative aims to foster collaboration with private industry partners. The flight marks only the sixth inaugural journey of a crewed spacecraft in US history, NASA Administrator Bill Nelson noted in a May news conference. “It started with Mercury, then with Gemini, then with Apollo, the space shuttle, then (SpaceX’s) Dragon — and now Starliner,” Nelson said. Williams also made history as the first woman to fly aboard such a mission. “This is another milestone in this extraordinary history of NASA,” Nelson said Wednesday after the launch. “And I want to give my personal congratulations to the whole team that went through a lot of trial and tribulation. But they had perseverance and that’s what we do at NASA. We don’t launch until it’s right.” Just after Wednesday’s launch, NASA officials shared that Williams and Wilmore may enjoy a slightly extended stay aboard the station. The earliest possible landing date is June 14. “We’ve got a prescribed landing date that goes along with this launch date, but I just want to emphasize that nobody should get too excited about that date,” said Ken Bowersox, associate administrator for NASA’s Space Operations Mission Directorate. “We have to have a lot of conditions that are just right before we bring the Starliner home and we’re going to wait till the conditions are right and we’ve accomplished the test objectives before we do that.” Weeks of troubleshooting A number of issues caused the previous crewed launch attempts, on May 6 and June 1, to be scrubbed. Two hours prior to the launch attempt on May 6, engineers identified an issue with a valve on the second stage, or upper portion, of the Atlas V rocket, which was built by United Launch Alliance, a joint venture between Boeing and Lockheed Martin. The entire stack, including the rocket and spacecraft, was rolled back from the launchpad for testing and repairs. Teams also worked through a&nbsp;small helium leak&nbsp;within the spacecraft service module and a “design vulnerability” in the propulsion system. After troubleshooting the initial helium leak in May, mission specialists found it did not pose a threat to the flight. During the launch countdown Wednesday morning, teams monitored the leak and reported no issues. Starliner was just 3 minutes and 50 seconds from liftoff Saturday afternoon, when an&nbsp;automatic hold was triggered&nbsp;by the ground launch sequencer, or the computer that launches the rocket. United Launch Alliance technicians and engineers assessed the ground support equipment over the weekend, examining three large computers housed inside a shelter at the base of the launchpad. Each computer is the same, providing triple redundancy to ensure the safe launch of crewed missions. Engineers isolated the issue that halted Saturday’s launch attempt to a single ground power supply within one of the computers, which provides power to the computer cards responsible for key countdown events, according to an update shared by NASA. They removed the computer and replaced it with a spare.");

    println!("{}", response);
}

fn test_audio() {
    services::chat_gpt_helpers::get_audio("Boeing’s Starliner made history by safely docking with the ISS, carrying NASA astronauts Butch Wilmore and Suni Williams. This marks the first time a Boeing spacecraft has transported humans. Despite helium leaks and thruster issues, the mission succeeded, with docking confirmed at 1:34 p.m. ET. After pressure equalized, Wilmore and Williams joined the ISS crew, now numbering nine, greeted with hugs and celebration. The mission is pivotal for Boeing, alongside SpaceX, in NASA's Commercial Crew Program. Challenges like manual piloting tests and helium management added drama but were expertly handled. The astronauts will spend eight days aboard, with plans to replace a critical water processing pump. This mission sets a course for Starliner's future certifications and deep space explorations.", "output_001.mp3");
}

fn test_subtitles() {
    let res = services::pico_voice_helpers::get_subtitles("output_001.mp3");
    println!("{:?}", res);
}

fn test_video() {
    let article = "{\"id\":\"ZMsMPWs49E\",\"headline\":\"Dornoch wins the Belmont Stakes, beating Derby and Preakness winners\",\"article_body\":\"Dornoch won the Belmont Stakes Saturday evening, with the race held for the first time in New York’s Saratoga Springs, 200 miles north of Belmont Park, which is under renovation. The Danny Gargan-trained horse broke late on the final turn to take the lead in the 10-horse field with Luis Saez aboard, besting the Kentucky Derby champion Mystik Dan and Preakness Stakes champion Seize The Grey, who were among the favorites for the final jewel in this year’s Triple Crown, along with post time favorite Sierra Leone, who finished third. Dornoch, who entered the race at 17-1 odds, held off a late charge from Mindframe and Sierra Leone to pull off the upset victory in the Triple Crown’s third leg. Dornoch was neck-and-neck with Seize The Grey until the final turn, where he pulled away from the Preakness winner. Mindframe finished the race in second. “It’s a huge race to win, I lost my voice a little bit, we were riding him down the lane pretty hard,” said trainer Danny Gargan after the race. “Speechless. He’s such a talented horse, we’ve thought so much of him all along. For him to get us here, and just be a part of it, it’s a special ride.” The victory is the first for Gargan in any Triple Crown race and second for Jockey Luis Saez. Dornoch is co-owned by 2008 World Series champion Jayson Werth. “I would put it right up there with winning on the biggest stage,” Werth said on the FOX broadcast. “Horse racing is the most underrated sport in the world, bar none. … This is as good as it gets in horse racing; this is as good as it gets in sports.” Saturday marked the first time in the 156-year history of the Belmont Stakes the race was hosted at a different track. It won’t be held at Belmont Park until at least 2026. No Triple Crown winner in 2024 It’s the sixth straight year a different horse has won each leg of the Triple Crown. The surprise Kentucky Derby champion Mystik Dan failed to keep the possibility of a Triple Crown success alive after finishing second at the Preakness Stakes last month. The race was won by Seize The Grey who held off Mystik Dan’s challenge on the home stretch. Despite the two champions lining up at the start on Saturday, the pre-race favorite for the Belmont Stakes was Sierra Leone. The 3-year-old was one of the horses involved in the Derby’s dramatic photo-finish, finishing second behind Mystik Dan after eventually breaking free from a congested group. The Derby, in a news release, said it was “the closest three-horse photo finish since 1947.” “If (Sierra Leone) had kept a straight line, it’s hard to imagine he wouldn’t have won,” trainer Chad Brown said, per the New York Post. “But that’s horse racing, a nice consolation prize would be the Belmont.” The field of stellar candidates racing at Saratoga Springs on Saturday had to contend with the change of track and distance for this year’s race. The Belmont Stakes has become known as “The Test of Champions” over the years, owing to the mile-and-a-half distance on the big sandy track at Belmont Park, but this year, however, the race was scaled back to a mile-and-a-quarter. The Saratoga Race Course is a shorter distance at 1-and-a-quarter miles, similar to the Kentucky Derby. Belmont Stakes entrants 1. Seize the Grey (8-1)2. Resilience (10-1)3. Mystik Dan (5-1)4. The Wine Steward (15-1)5. Antiquarian (12-1)6. Dornoch (15-1)7. Protective (20-1)8. Honor Marie (12-1)9. Sierra Leone (9-5)10. Mindframe (7-2)\",\"description\":\"Dornoch won the Belmont Stakes Saturday evening, with the race held for the first time in New York’s Saratoga Springs, 200 miles north of Belmont Park, which is under renovation.\",\"genre\":[\"sport\"],\"images\":[\"https://media.cnn.com/api/v1/images/stellar/prod/ap24160837912884.jpg?c=original\",\"https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2156698139.jpg?c=original\",\"https://media.cnn.com/api/v1/images/stellar/prod/ap24157590554010.jpg?c=original\"]}";

    let subtitles = "[{\"id\":0,\"seek\":0.0,\"start\":0.064,\"end\":0.096,\"text\":\"in\"},{\"id\":0,\"seek\":0.0,\"start\":0.16,\"end\":0.192,\"text\":\"a\"},{\"id\":0,\"seek\":0.0,\"start\":0.256,\"end\":0.512,\"text\":\"thrilling\"},{\"id\":0,\"seek\":0.0,\"start\":0.576,\"end\":0.864,\"text\":\"twist\"},{\"id\":0,\"seek\":0.0,\"start\":1.088,\"end\":1.216,\"text\":\"door\"},{\"id\":0,\"seek\":0.0,\"start\":1.28,\"end\":1.44,\"text\":\"not\"},{\"id\":0,\"seek\":0.0,\"start\":1.536,\"end\":1.824,\"text\":\"clinched\"},{\"id\":0,\"seek\":0.0,\"start\":1.856,\"end\":1.92,\"text\":\"the\"},{\"id\":0,\"seek\":0.0,\"start\":1.984,\"end\":2.304,\"text\":\"Belmont\"},{\"id\":0,\"seek\":0.0,\"start\":2.336,\"end\":2.624,\"text\":\"stakes\"},{\"id\":0,\"seek\":0.0,\"start\":2.752,\"end\":2.816,\"text\":\"at\"},{\"id\":0,\"seek\":0.0,\"start\":2.88,\"end\":3.36,\"text\":\"Saratoga\"},{\"id\":0,\"seek\":0.0,\"start\":3.424,\"end\":3.808,\"text\":\"springs\"},{\"id\":0,\"seek\":0.0,\"start\":4.192,\"end\":4.192,\"text\":\"a\"},{\"id\":0,\"seek\":0.0,\"start\":4.288,\"end\":4.672,\"text\":\"first-time\"},{\"id\":0,\"seek\":0.0,\"start\":4.768,\"end\":4.992,\"text\":\"venue\"},{\"id\":0,\"seek\":0.0,\"start\":5.12,\"end\":5.184,\"text\":\"two\"},{\"id\":0,\"seek\":0.0,\"start\":5.28,\"end\":5.472,\"text\":\"hundred\"},{\"id\":0,\"seek\":0.0,\"start\":5.536,\"end\":5.76,\"text\":\"miles\"},{\"id\":0,\"seek\":0.0,\"start\":5.824,\"end\":5.984,\"text\":\"north\"},{\"id\":0,\"seek\":0.0,\"start\":6.048,\"end\":6.08,\"text\":\"of\"},{\"id\":0,\"seek\":0.0,\"start\":6.144,\"end\":6.464,\"text\":\"Belmont\"},{\"id\":0,\"seek\":0.0,\"start\":6.528,\"end\":6.72,\"text\":\"park\"},{\"id\":0,\"seek\":0.0,\"start\":7.008,\"end\":7.392,\"text\":\"undergoing\"},{\"id\":0,\"seek\":0.0,\"start\":7.456,\"end\":7.968,\"text\":\"renovations\"},{\"id\":0,\"seek\":0.0,\"start\":8.384,\"end\":8.448,\"text\":\"the\"},{\"id\":0,\"seek\":0.0,\"start\":8.512,\"end\":8.896,\"text\":\"seventeen\"},{\"id\":0,\"seek\":0.0,\"start\":8.992,\"end\":9.088,\"text\":\"one\"},{\"id\":0,\"seek\":0.0,\"start\":9.152,\"end\":9.312,\"text\":\"long\"},{\"id\":0,\"seek\":0.0,\"start\":9.408,\"end\":9.632,\"text\":\"shot\"},{\"id\":0,\"seek\":0.0,\"start\":9.76,\"end\":10.208,\"text\":\"triumphed\"},{\"id\":0,\"seek\":0.0,\"start\":10.272,\"end\":10.4,\"text\":\"over\"},{\"id\":0,\"seek\":0.0,\"start\":10.496,\"end\":10.816,\"text\":\"favorites\"},{\"id\":0,\"seek\":0.0,\"start\":10.88,\"end\":11.168,\"text\":\"mystic\"},{\"id\":0,\"seek\":0.0,\"start\":11.264,\"end\":11.456,\"text\":\"Dan\"},{\"id\":0,\"seek\":0.0,\"start\":11.808,\"end\":12.096,\"text\":\"seized\"},{\"id\":0,\"seek\":0.0,\"start\":12.128,\"end\":12.16,\"text\":\"the\"},{\"id\":0,\"seek\":0.0,\"start\":12.224,\"end\":12.352,\"text\":\"gray\"},{\"id\":0,\"seek\":0.0,\"start\":12.704,\"end\":12.8,\"text\":\"and\"},{\"id\":0,\"seek\":0.0,\"start\":12.864,\"end\":13.152,\"text\":\"Sierra\"},{\"id\":0,\"seek\":0.0,\"start\":13.184,\"end\":13.504,\"text\":\"Leone\"},{\"id\":0,\"seek\":0.0,\"start\":13.664,\"end\":13.76,\"text\":\"with\"},{\"id\":0,\"seek\":0.0,\"start\":13.824,\"end\":13.824,\"text\":\"a\"},{\"id\":0,\"seek\":0.0,\"start\":13.888,\"end\":14.048,\"text\":\"late\"},{\"id\":0,\"seek\":0.0,\"start\":14.112,\"end\":14.432,\"text\":\"burst\"},{\"id\":0,\"seek\":0.0,\"start\":15.04,\"end\":15.168,\"text\":\"this\"},{\"id\":0,\"seek\":0.0,\"start\":15.264,\"end\":15.456,\"text\":\"race\"},{\"id\":0,\"seek\":0.0,\"start\":15.648,\"end\":15.712,\"text\":\"the\"},{\"id\":0,\"seek\":0.0,\"start\":15.808,\"end\":16.064,\"text\":\"final\"},{\"id\":0,\"seek\":0.0,\"start\":16.128,\"end\":16.352,\"text\":\"jewel\"},{\"id\":0,\"seek\":0.0,\"start\":16.416,\"end\":16.448,\"text\":\"in\"},{\"id\":0,\"seek\":0.0,\"start\":16.512,\"end\":16.544,\"text\":\"the\"},{\"id\":0,\"seek\":0.0,\"start\":16.576,\"end\":16.8,\"text\":\"triple\"},{\"id\":0,\"seek\":0.0,\"start\":16.864,\"end\":17.088,\"text\":\"crown\"},{\"id\":0,\"seek\":0.0,\"start\":17.376,\"end\":17.472,\"text\":\"saw\"},{\"id\":0,\"seek\":0.0,\"start\":17.6,\"end\":17.696,\"text\":\"no\"},{\"id\":0,\"seek\":0.0,\"start\":17.824,\"end\":18.112,\"text\":\"single\"},{\"id\":0,\"seek\":0.0,\"start\":18.176,\"end\":18.368,\"text\":\"victor\"},{\"id\":0,\"seek\":0.0,\"start\":18.432,\"end\":18.496,\"text\":\"for\"},{\"id\":0,\"seek\":0.0,\"start\":18.56,\"end\":18.592,\"text\":\"the\"},{\"id\":0,\"seek\":0.0,\"start\":18.656,\"end\":18.88,\"text\":\"sixth\"},{\"id\":0,\"seek\":0.0,\"start\":18.912,\"end\":19.424,\"text\":\"consecutive\"},{\"id\":0,\"seek\":0.0,\"start\":19.488,\"end\":19.616,\"text\":\"year\"},{\"id\":0,\"seek\":0.0,\"start\":20.192,\"end\":20.448,\"text\":\"trainer\"},{\"id\":0,\"seek\":0.0,\"start\":20.512,\"end\":20.704,\"text\":\"Danny\"},{\"id\":0,\"seek\":0.0,\"start\":20.768,\"end\":21.056,\"text\":\"Gargan\"},{\"id\":0,\"seek\":0.0,\"start\":21.28,\"end\":21.728,\"text\":\"emotional\"},{\"id\":0,\"seek\":0.0,\"start\":21.792,\"end\":21.856,\"text\":\"and\"},{\"id\":0,\"seek\":0.0,\"start\":21.888,\"end\":22.336,\"text\":\"triumphant\"},{\"id\":0,\"seek\":0.0,\"start\":22.528,\"end\":22.816,\"text\":\"reveled\"},{\"id\":0,\"seek\":0.0,\"start\":22.848,\"end\":22.88,\"text\":\"in\"},{\"id\":0,\"seek\":0.0,\"start\":22.912,\"end\":23.008,\"text\":\"his\"},{\"id\":0,\"seek\":0.0,\"start\":23.072,\"end\":23.264,\"text\":\"first\"},{\"id\":0,\"seek\":0.0,\"start\":23.328,\"end\":23.552,\"text\":\"triple\"},{\"id\":0,\"seek\":0.0,\"start\":23.584,\"end\":23.776,\"text\":\"crown\"},{\"id\":0,\"seek\":0.0,\"start\":23.84,\"end\":24.128,\"text\":\"victory\"},{\"id\":0,\"seek\":0.0,\"start\":24.32,\"end\":24.512,\"text\":\"while\"},{\"id\":0,\"seek\":0.0,\"start\":24.608,\"end\":24.864,\"text\":\"jockey\"},{\"id\":0,\"seek\":0.0,\"start\":24.96,\"end\":25.536,\"text\":\"lewises\'\"},{\"id\":0,\"seek\":0.0,\"start\":25.632,\"end\":25.792,\"text\":\"earned\"},{\"id\":0,\"seek\":0.0,\"start\":25.824,\"end\":25.888,\"text\":\"his\"},{\"id\":0,\"seek\":0.0,\"start\":25.952,\"end\":26.272,\"text\":\"second\"},{\"id\":0,\"seek\":0.0,\"start\":26.752,\"end\":27.104,\"text\":\"co-owner\"},{\"id\":0,\"seek\":0.0,\"start\":27.232,\"end\":27.488,\"text\":\"Jason\"},{\"id\":0,\"seek\":0.0,\"start\":27.552,\"end\":27.712,\"text\":\"worth\"},{\"id\":0,\"seek\":0.0,\"start\":27.808,\"end\":28.096,\"text\":\"compared\"},{\"id\":0,\"seek\":0.0,\"start\":28.128,\"end\":28.16,\"text\":\"the\"},{\"id\":0,\"seek\":0.0,\"start\":28.224,\"end\":28.448,\"text\":\"wind\"},{\"id\":0,\"seek\":0.0,\"start\":28.48,\"end\":28.544,\"text\":\"to\"},{\"id\":0,\"seek\":0.0,\"start\":28.576,\"end\":28.672,\"text\":\"his\"},{\"id\":0,\"seek\":0.0,\"start\":28.736,\"end\":28.832,\"text\":\"two\"},{\"id\":0,\"seek\":0.0,\"start\":28.928,\"end\":29.184,\"text\":\"thousand\"},{\"id\":0,\"seek\":0.0,\"start\":29.248,\"end\":29.312,\"text\":\"and\"},{\"id\":0,\"seek\":0.0,\"start\":29.344,\"end\":29.44,\"text\":\"ate\"},{\"id\":0,\"seek\":0.0,\"start\":29.536,\"end\":29.728,\"text\":\"world\"},{\"id\":0,\"seek\":0.0,\"start\":29.824,\"end\":30.08,\"text\":\"series\"},{\"id\":0,\"seek\":0.0,\"start\":30.176,\"end\":30.496,\"text\":\"triumph\"},{\"id\":0,\"seek\":0.0,\"start\":30.848,\"end\":31.264,\"text\":\"highlighting\"},{\"id\":0,\"seek\":0.0,\"start\":31.328,\"end\":31.488,\"text\":\"horse\"},{\"id\":0,\"seek\":0.0,\"start\":31.584,\"end\":31.904,\"text\":\"racing\'s\"},{\"id\":0,\"seek\":0.0,\"start\":32.064,\"end\":32.512,\"text\":\"underrated\"},{\"id\":0,\"seek\":0.0,\"start\":32.576,\"end\":32.864,\"text\":\"drama\"},{\"id\":0,\"seek\":0.0,\"start\":33.472,\"end\":33.504,\"text\":\"the\"},{\"id\":0,\"seek\":0.0,\"start\":33.6,\"end\":34.144,\"text\":\"historical\"},{\"id\":0,\"seek\":0.0,\"start\":34.176,\"end\":34.272,\"text\":\"one\"},{\"id\":0,\"seek\":0.0,\"start\":34.336,\"end\":34.56,\"text\":\"hundred\"},{\"id\":0,\"seek\":0.0,\"start\":34.592,\"end\":34.656,\"text\":\"and\"},{\"id\":0,\"seek\":0.0,\"start\":34.688,\"end\":34.88,\"text\":\"fifty\"},{\"id\":0,\"seek\":0.0,\"start\":34.976,\"end\":35.136,\"text\":\"six\"},{\"id\":0,\"seek\":0.0,\"start\":35.232,\"end\":35.328,\"text\":\"year\"},{\"id\":0,\"seek\":0.0,\"start\":35.392,\"end\":35.488,\"text\":\"old\"},{\"id\":0,\"seek\":0.0,\"start\":35.552,\"end\":35.776,\"text\":\"event\"},{\"id\":0,\"seek\":0.0,\"start\":36.0,\"end\":36.288,\"text\":\"shifted\"},{\"id\":0,\"seek\":0.0,\"start\":36.352,\"end\":36.384,\"text\":\"to\"},{\"id\":0,\"seek\":0.0,\"start\":36.448,\"end\":36.48,\"text\":\"a\"},{\"id\":0,\"seek\":0.0,\"start\":36.544,\"end\":36.8,\"text\":\"shorter\"},{\"id\":0,\"seek\":0.0,\"start\":36.864,\"end\":37.024,\"text\":\"mile\"},{\"id\":0,\"seek\":0.0,\"start\":37.088,\"end\":37.152,\"text\":\"and\"},{\"id\":0,\"seek\":0.0,\"start\":37.184,\"end\":37.184,\"text\":\"a\"},{\"id\":0,\"seek\":0.0,\"start\":37.216,\"end\":37.44,\"text\":\"quarter\"},{\"id\":0,\"seek\":0.0,\"start\":37.536,\"end\":37.728,\"text\":\"track\"},{\"id\":0,\"seek\":0.0,\"start\":38.048,\"end\":38.304,\"text\":\"shaking\"},{\"id\":0,\"seek\":0.0,\"start\":38.368,\"end\":38.432,\"text\":\"up\"},{\"id\":0,\"seek\":0.0,\"start\":38.496,\"end\":38.528,\"text\":\"the\"},{\"id\":0,\"seek\":0.0,\"start\":38.592,\"end\":39.072,\"text\":\"competition\"}]";

    let article: NewsArticle = serde_json::from_str(&article).unwrap();
    let subtitles: Vec<TranscribeSegment> = serde_json::from_str(&subtitles).unwrap();

    ffmpeg_helper::generate_video(article, subtitles);
}

fn pipeline(article_url: &str) {
    // ===== SCRAPE WEBSITE =====
    println!("Scraping Website");
    let news_article = services::news_crawler::scrape_cnn_news(article_url);

    if let Some(article) = news_article {
        println!("Website scraped {}", article.id);

        // ===== GENERATE SUMMARY =====
        let summary = services::chat_gpt_helpers::get_summary(&article.article_body);
        println!("Summary generated");

        // ===== GENERATE AUDIO =====
        let audio_path = article.get_audio_path();
        services::chat_gpt_helpers::get_audio(&summary, &audio_path);
        println!("Audio saved at {}", audio_path);

        // ===== GENERATE SUBTITLES =====
        let subtitles = services::pico_voice_helpers::get_subtitles(&audio_path);
        println!("Subtitles generated");

        println!("{}", serde_json::to_string(&article).unwrap());
        println!("{}", serde_json::to_string(&subtitles).unwrap());

        // ===== GENERATE VIDEO =====
        services::ffmpeg_helper::generate_video(article, subtitles);
        println!("Video generated");
    }
}

async fn test_posts(subreddit: &str) {
    let posts = services::reddit_helpers::get_top_posts(subreddit, "day").await;
    for post in posts {
        let post_id = post.data.get_post_id();
        if post.data.is_eligible_for_insta()
            && !DB_SERVICE.get().unwrap().check_post_exists(&post_id).await
        {
            let download_result = services::reddit_helpers::download_reddit_video(&post).await;
            match download_result {
                Ok(_) => {
                    services::ffmpeg_helper::resize_video(
                        &post.data.get_video_path(),
                        "flipdascript-alpha.png",
                        &post.data.get_converted_video_path(),
                    );
                    services::ffmpeg_helper::generate_cover(
                        &post.data.get_converted_video_path(),
                        &post.data.get_video_cover_path(),
                    );
                    if services::instagram_helpers::upload_to_instagram(&post).await {
                        DB_SERVICE.get().unwrap().add_post(&post_id).await;

                        let _ = utils::delete_file(&post.data.get_video_path());
                        let _ = utils::delete_file(&&post.data.get_converted_video_path());
                        let _ = utils::delete_file(&&post.data.get_video_cover_path());
                    }
                    break;
                }
                Err(__) => {}
            }
        }
    }
}

pub static CONFIG: OnceCell<Config> = OnceCell::new();
pub static DB_SERVICE: OnceCell<DbService> = OnceCell::new();

#[tokio::main]
async fn main() {
    // SET ARGS
    let args: Vec<String> = env::args().collect();
    let subreddit = &args[1];
    let _ = CONFIG.set(Config::from_env(&format!("{}.env", subreddit)));

    // INIT DB
    let _ = DB_SERVICE.set(DbService::new().await);

    test_posts(subreddit).await;
}
