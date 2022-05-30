#!/bin/sh

blueBG=$(tput setab 4)
blackFG=$(tput setaf 0)
reset=$(tput sgr0)
action=$(echo "${blueBG}${blackFG}"[ACTION]"${reset}")

dfx identity new jack
dfx identity use jack
dfx canister call dscvr_takehome_rust create_user "(\"jack\")"
dfx canister call dscvr_takehome_rust create_post "(\"Hello World!\")"
dfx canister call dscvr_takehome_rust create_post "(\"WAGMI\")"
dfx canister call dscvr_takehome_rust create_post "(\"GM\")"

dfx identity new bill
dfx identity use bill
dfx canister call dscvr_takehome_rust create_user "(\"bill\")"
dfx canister call dscvr_takehome_rust create_post "(\"Wen airdrop?\")"
dfx canister call dscvr_takehome_rust create_post "(\"NAGMI\")"
dfx canister call dscvr_takehome_rust create_post "(\"My airdrops are empty, can you look into it?\")"

printf "\n%s Switching back to the default identity.\n" "${action}"

dfx identity use default