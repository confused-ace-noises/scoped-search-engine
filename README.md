# ICEPICK
Icepick is a revolutionary search which target audience is advanced users.

## Usage 
Icepick isn't, a normal search engine: whereas in other search engines, the website crawling is done behind the scene, often hindering result usefulness because of the big haystack, Icepick gives the user the freedom of controlling this step too: provide a url from which to start crawling, a depth to reach, and a query to search for; Icepick will only give you results from those crawled sites.

## When is this useful?
Icepick is very useful in research scenarios: maybe you need to search something very obscure that other search engines will likely never reach, or need to limit your search because other search engines will just give too many results; Icepick in this scenarios becomes the star of the show, letting the user control the crawling step too. 

## Results and score calculation
The results given by icepick are ordered from most to least relevant, based on a very simple equation:

`website_crawl_depth * website_crawl_depth_coefficient + n_mentions * n_mentions_coefficient + n_query_matches * n_query_matches_coefficient`

where:
    - website_crawl_depth is the depth of the crawl at which the website is first found: deeper sites will stray   farther from the topic of the starting url;
    - n_mentions is how many times other websites linked to the website considered: the more times it is mentioned, the more likely it is to be reliable.
    - n_query_matches: simply, how many matches of the query given are present in that website.

the coefficients of each one are decided by the user based on their specific needs. The default ones are:
    - website_crawl_depth_coefficient: -0.7
    - n_mentions_coefficient: 1.7
    - n_query_matches_coefficient: 2.5

#### Search Examples
The coefficients to each value are variable because they can be tailored to the kind of search the user needs to carry out.
examples:
1) The user needs to get results that *don't* contain the query given:
    - website_crawl_depth_coefficient: 0.2
    - n_mentions_coefficient: 2.0
    - n_query_matches_coefficient: -3.2

2) The user needs to get results that remain as pertinent to the starting url that contain the given query:
    - website_crawl_depth_coefficient: -3.7
    - n_mentions_coefficient: 1.3
    - n_query_matches_coefficient: 2.9

3) The user needs to find most obscure websites in the crawl done, the ones that are linked to once, maybe twice:
    - website_crawl_depth_coefficient: -0.3
    - n_mentions_coefficient: -3.5
    - n_query_matches_coefficient: 1.2

These were parameters that gave the testers the best results for each search purpose.

## Installing Icepick
Icepick's main part is stationed in the `icepick-backend` directory; to install it, just clone it, and then upon having installed `cargo`, running `cargo run --release`. This will start the sever Icepick is based on. The server can be interacted with through POST calls to the path `/api/search`; docs are in development.

The user part of the search engine is still in heavy development, and the only current stable and mostly fleshed out way of interacting with the server is with CLI. The CLI for Icepick is located in `cli`. To use it, clone said directory, and then after having installed cargo, run `cargo run -- <args here>`. For example: `cargo run -- --help`.

(P.S.: a webui way of interacting with icepick is on its way.)