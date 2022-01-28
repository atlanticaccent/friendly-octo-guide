var searchIndex = JSON.parse('{\
"truelayer_coding_challenge":{"doc":"Cached Pokemon description retrieval and subsequent fun …","t":[0,0,0,0,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,0,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,5,5,12,8,13,13,3,13,13,18,13,4,8,13,18,8,4,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,10,10,10,5,11,10,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12],"n":["api","models","server","util","API","borrow","borrow_mut","clone","clone_into","disable_https","from","get_pokeapi_url","get_pokemon","get_translation_url","into","new","override_uri","to_owned","translate","try_from","try_into","type_id","vzip","poke_models","translation_models","FlavourText","NamedAPIResource","PokemonResponse","PokemonSpecies","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","description","deserialize","deserialize","deserialize","flavor_text","from","from","from","from","get_first_description","habitat","habitat","into","into","into","into","is_legendary","is_legendary","language","name","name","name","serialize","set_description","to_owned","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","vzip","vzip","vzip","vzip","Contents","TranslationUnit","borrow","borrow","borrow_mut","borrow_mut","contents","deserialize","deserialize","from","from","into","into","translated","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip","advanced_handler","basic_handler","router","0","CacheWrapper","Http","Hyper","MokaCache","NoDescription","None","POKEAPI","Parse","PokError","PokeClient","Shakespeare","TRANSLATION_API","TranslationClient","TranslationType","Unavailable","Warp","Yoda","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","eq","equivalent","fmt","fmt","from","from","from","from","from","from","from","get","get","get_pokeapi_url","get_pokemon","get_translation_url","handle_reject","hash","insert","insert","into","into","into","source","to_owned","to_owned","to_string","to_string","translate","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","0","0","0","0","0"],"q":["truelayer_coding_challenge","","","","truelayer_coding_challenge::api","","","","","","","","","","","","","","","","","","","truelayer_coding_challenge::models","","truelayer_coding_challenge::models::poke_models","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","truelayer_coding_challenge::models::translation_models","","","","","","","","","","","","","","","","","","","","","","truelayer_coding_challenge::server","","","truelayer_coding_challenge::util","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","truelayer_coding_challenge::util::PokError","","","",""],"d":["","","","","An “API” that can connect to a given API and make …","","","","","Disable https connectivity","","","","","","","Set the URI override - this host will be contacted instead …","","","","","","","","","A flavor text as returned by Pokeapi","A Name API Resource","The response this API will return following a successful …","Serde model representing a response from Pokeapi.","","","","","","","","","","","Get a reference to the pokemon’s description.","","","","Get a reference to the actual flavor text.","","","","","Get an option that may contain a reference to the first …","Get a reference to the pokemon species’s habitat.","Get a reference to the pokemon’s habitat.","","","","","Get the pokemon’s legendary status.","Get whether the pokemon is legendary.","Get a reference to the flavour text’s language.","Get a reference to the pokemon’s name.","Get a reference to the named apiresource’s value.","Get a reference to the pokemon’s name.","","","","","","","","","","","","","","","","","","","","","The actual translated string, as wrapped in a JSON object","The response returned from api.funtranslations","","","","","Get a reference to the translation unit’s contents.","","","","","","","Get a reference to the contents’s translated string.","","","","","","","","","Filter for “advanced”, translation API requests","Filter for “basic” non-translation API requests","Full router of available public API endpoints","","Trait defining cache insertion and get functions","","","Non-test implementation of the CacheWrapper trait.","","","API host address - aka: authority","","Potential errors that can ocurr during running","Trait defining the functions an API object needs to …","","API host address - aka: authority","Trait defining the methods an API object needs to contact …","The type of translation that is being requested","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Handle errors raised at runtime and generate appropriate …","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,2,3,4,5,2,3,4,5,5,5,5,2,3,4,4,2,3,4,5,2,2,5,2,3,4,5,2,5,4,2,3,5,5,5,5,2,3,4,5,5,2,3,4,5,2,3,4,5,2,3,4,5,0,0,6,7,6,7,6,6,7,6,7,6,7,7,6,7,6,7,6,7,6,7,0,0,0,8,0,9,9,0,9,10,11,9,0,0,10,12,0,0,9,9,10,8,10,9,8,10,9,8,10,8,10,10,10,9,9,8,10,9,9,9,9,9,13,8,11,11,12,0,10,13,8,8,10,9,9,8,10,10,9,12,8,10,9,8,10,9,8,10,9,8,10,9,14,15,16,17,18],"f":[null,null,null,null,null,[[]],[[]],[[],["api",3]],[[]],[[]],[[]],[[],["string",3]],[[["string",3]],["pin",3,[["box",3,[["future",8]]]]]],[[],["string",3]],[[]],[[]],[[["string",3]]],[[]],[[["pokemonresponse",3],["translationtype",4]],["pin",3,[["box",3,[["future",8]]]]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["pokemonresponse",3]],[[]],[[],["str",15]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["str",15]],[[]],[[]],[[]],[[]],[[["str",15]],["option",4,[["string",3]]]],[[],["str",15]],[[],["str",15]],[[]],[[]],[[]],[[]],[[],["bool",15]],[[],["bool",15]],[[],["namedapiresource",3]],[[],["str",15]],[[],["str",15]],[[],["str",15]],[[],["result",4]],[[["string",3]]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["pokemonspecies",3]],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]],[[]],null,null,[[]],[[]],[[]],[[]],[[],["contents",3]],[[],["result",4]],[[],["result",4]],[[]],[[]],[[]],[[]],[[],["str",15]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[["pokemonresponse",3]]],[[["string",3]]],[[]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["mokacache",3]],[[],["translationtype",4]],[[]],[[]],[[["translationtype",4]],["bool",15]],[[],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["error",3]]],[[["error",3]]],[[["error",3]]],[[["error",3]]],[[]],[[],["option",4]],[[],["option",4]],[[],["string",3]],[[["string",3]],["pin",3,[["box",3,[["future",8]]]]]],[[],["string",3]],[[["rejection",3]]],[[]],[[],["pin",3,[["box",3,[["future",8]]]]]],[[],["pin",3,[["box",3,[["future",8]]]]]],[[]],[[]],[[]],[[],["option",4,[["error",8]]]],[[]],[[]],[[],["string",3]],[[],["string",3]],[[["pokemonresponse",3],["translationtype",4]],["pin",3,[["box",3,[["future",8]]]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]],null,null,null,null,null],"p":[[3,"API"],[3,"PokemonSpecies"],[3,"NamedAPIResource"],[3,"FlavourText"],[3,"PokemonResponse"],[3,"TranslationUnit"],[3,"Contents"],[3,"MokaCache"],[4,"PokError"],[4,"TranslationType"],[8,"PokeClient"],[8,"TranslationClient"],[8,"CacheWrapper"],[13,"Hyper"],[13,"Http"],[13,"Unavailable"],[13,"Parse"],[13,"Warp"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};