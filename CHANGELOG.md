# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - ReleaseDate

<!-- [START AUTO UPDATE] -->
<!-- Please keep comment here to allow auto-update -->
## [0.1.2-66835a3] - 2023-10-19

Feature/cu 8692w9z59 : Create all the necessary DTO structures for the NetworkBandwith chart (#10)

* feature/CU-8692w9z59: Add struct bandwith_bucket, network_bandwith and network_bandwith_request for bandwith chart
<!-- [END AUTO UPDATE] -->
## [0.1.2-5f6fcc3] - 2023-10-16

Feature/cu 8692vgfbd (#8)

*  added overview_dashboard_request

*  udpated net-proto-api to 0.1.1-da85a0a

*  fixed bad method name in dashboard_request
## [0.1.2-90bf1ae] - 2023-10-09

Feature/cu 8692vgfbd: added structures for dashboards (#7)

* updated APIs by changes net-proto-api

* added dashboard DTO structures
## [0.1.2-8fb717a] - 2023-10-09

feature/CU-8692vnf7e: updated APIs by changes net-proto-api (#6)

*  updated net-api components after adding supertrait API and trait Typed into net-proto-api
## [0.1.2-17ea57a] - 2023-10-03

Feature/cu 8692u7hf4: updated net-timescale-api (#3)

* feature/CU-8692u7hf4: changed names of the field in graph_node.rs (id -> node_id, aggregator -> agent_id), fixed endec tests in graph_node.rs and in network_graph.rs

* feature/CU-8692u7hf4: added factor list into graph_edge.rs, updated network_graph.rs

* feature/CU-8692u7hf4: renamed factor to communication_types

* feature/CU-8692u7hf4: optimized decoding in network_graph.rs

* feature/CU-8692u7hf4: removed unused import
## [0.1.2-86d583b] - 2023-09-28

feature/update-push-prereleases.yml: updated push-prerelease.yml to m… (#4)

* feature/update-push-prereleases.yml: updated push-prerelease.yml to make it workflow trigger after pushing into pull requests
## [0.1.2-42099e1] - 2023-09-25

feature/CU-8692te2td: updated ci scripts (#2)

* feature/CU-8692te2td: updated ci scripts, changed owner's credentials to the bot's ones
## [0.1.2-dd83adc] - 2023-09-12

Feature/cu 8692q8rzy: moved net-*-apis to net-api repo (#1)

### Added
- github ci flows
- net-timescale-api into cargo workspace
- net-agent-api into cargo workspace
- .isl files in net-timesacle-api and net-agent-api, udpated net-proto-api version to 0.1.1-00a5b83
- CHANGELOG.md

### Updated

- README,md file
- net-agent-api version to 0.1.2 to make it identical with net-timescale-api

## [0.1.0] - 2023-09-12

### Added
Initial change log