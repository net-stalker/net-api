# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - ReleaseDate

<!-- [START AUTO UPDATE] -->
<!-- Please keep comment here to allow auto-update -->
## [0.1.2-6d41626] - 2024-02-24

NS-112/add-filters-to-bpp: filters for network bandwidth per protocol chart API (#25)

* implemented filters for network bandwidth per protocol chart api, added the strcuture to request dto structure

* modified net-reporter-api version to 0.3.0
<!-- [END AUTO UPDATE] -->
## [0.1.2-3d11b49] - 2024-02-21

Ns 111/add filters for network graph (#24)

* implemented filter structure for network graph

* added network graph filters to network graph request structure

* updated graph_edge and graph_node dto structures, removed unused fields and added value property for edge

* updated net-reporter-api version to 0.2.0
## [0.1.2-8babb55] - 2024-02-21

Ns 110/add filters for bpe: added DTO structure for bandwidth per endpoint filters  (#23)

* implemented bandwidth per endpoint filters strucutre

* added filters for bandwidth per endpoint

* updated net-reporter-api version to 0.1.32

## [0.1.2-b6a7954] - 2024-02-21

Ns 102/add filters chart requests: added filters structure for network bandwidth  (#22)

* implemented network bandwidth filters dto structures

* added network bandwidth filters structure into network bandwidth request

* updated structures by adding Option to filters modes

* updated request network bandwidth structure

* updated net-reporter-api version to 0.1.31
## [0.1.2-fa31716] - 2024-02-19

NS-83/network-bandwidth-per-protocol (#21)

* Add network_bandwidth_per_protocol .is file

* Add ProtocolDTO

* Add NetworkBandwidthPerProtocolDTO

* Add NetworkBandwidthPerProtocolRequestDTO
## [0.1.2-0b799f0] - 2024-02-07

NS-57/network-overview-dashboard-filters (#18)

* added-DTO-strucutres-for-network-overview-dashboard-filters

* updated net-reporter-api version

* removed bytes_rec and bytes_sent to a single field total_bytes
## [0.1.2-cc0dc49] - 2024-02-07

Feature/NS-50/managing-crates-to-be-pushed-to-cratesio (#20)

* Added att the essentials for publishing on crates.io

* Split timescale-api into two parts
## [0.1.2-d1bc25c] - 2024-02-06

feature/CU-8693cxbcn: deleted ion text feature due to its uselessness (#17)

* Delete ion text feature due to its uselessness
## [0.1.2-0de1605] - 2023-11-22

feature/CU-86932v6mc: removed manual reading/writing from decode/encode methods in composed dto structures (#14)

* removed manual reading/writing from decode/encode methods in composed dto structures
## [0.1.2-f7e482d] - 2023-10-26

feature/CU-86930959c: Add missing APIs + refactor (#13)

* feature/CU-86930959c: Add missing APIs + small refactor
## [0.1.2-a31021f] - 2023-10-25

Feature/cu 8692xqew1: Refactor all the apis (DTO structs) (#11)

* feature/CU-8692xqew1: Refactor all the APIs (DTOs) in net-timescale
## [0.1.2-5b5436c] - 2023-10-25

feature/CU-8692w9z59: Add the missing .isl file for the bandwith_request (#12)

* feature/CU-8692w9z59: Add the missing .isl file for the bandwith_request

* feature/CU-8692w9z59: Rename bandwith to bandwidth
## [0.1.2-20a413f] - 2023-10-24

Feature/cu 8692xghpp: implemented structures for bandwidth per endpoint chart (#9)

### Implemented

* EndpointDTO structure

* BandwidthPerEndpointDTO structure

* BandwidthPerEndpointRequestDTO structure

### Added

* API trait implementations for bandwidth per endpoint structures
## [0.1.2-66835a3] - 2023-10-19

Feature/cu 8692w9z59 : Create all the necessary DTO structures for the NetworkBandwith chart (#10)

* feature/CU-8692w9z59: Add struct bandwith_bucket, network_bandwith and network_bandwith_request for bandwith chart
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