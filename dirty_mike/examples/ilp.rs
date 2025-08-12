fn main() {
    let work1 = dirty_mike::topdown(|| {
        let _v: serde_json::Value = serde_json::from_str(DATA).unwrap();
    })
    .unwrap();

    let work2 = dirty_mike::topdown(|| {
        let _v: toml::Value = toml::from_str(DATA_TOML).unwrap();
    })
    .unwrap();

    dbg!(work1.stats());
    dbg!(work2.stats());
}

static DATA: &str = r#"
{
  "randomStrings": [
    "xkjf94jf2kl3j4lk5j6lk7j8lk9j0lk1j2lk3j4lk5j6lk7j8lk9j0lk1j2lk3j4lk5j6lk7j8lk9j0",
    "mqwerty789456123qwertyuiopasdfghjklzxcvbnm098765432109876543210987654321",
    "aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzzaabbccddeeffgghhii",
    "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt",
    "abcdefghijklmnopqrstuvwxyz0123456789abcdefghijklmnopqrstuvwxyz0123456789",
    "fizzbuzzfoobarbazbingbangboomhelloworldgoodbyemoonhellostarsnicetomeetyou",
    "qwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjkl",
    "1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t1u2v3w4x5y6z7a8b9c0d1e2f3g4h"
  ],
  "nestedObjects": {
    "level1": {
      "level2": {
        "level3": {
          "level4": {
            "data": "very deep nested data with long string values and meaningless content",
            "numbers": [42, 3.14159, 2.71828, 1.41421, 1.61803, 0.57721, 2.50290, 4.66920],
            "booleans": [true, false, true, true, false, false, true, false, true, false]
          }
        }
      }
    },
    "anotherBranch": {
      "moreData": "this is just filler text to make the json larger and more complex",
      "arrayOfObjects": [
        {"id": "abc123", "name": "widget", "value": 99.99, "active": true},
        {"id": "def456", "name": "gadget", "value": 149.99, "active": false},
        {"id": "ghi789", "name": "doodad", "value": 24.99, "active": true},
        {"id": "jkl012", "name": "thingamajig", "value": 199.99, "active": false}
      ]
    }
  },
  "longArray": [
    "item001", "item002", "item003", "item004", "item005", "item006", "item007", "item008",
    "item009", "item010", "item011", "item012", "item013", "item014", "item015", "item016",
    "item017", "item018", "item019", "item020", "item021", "item022", "item023", "item024",
    "item025", "item026", "item027", "item028", "item029", "item030", "item031", "item032"
  ],
  "metadata": {
    "created": "2024-01-01T00:00:00Z",
    "modified": "2024-12-31T23:59:59Z",
    "version": "1.0.0",
    "author": "DataGenerator3000",
    "description": "This is a large blob of nonsense JSON data created for testing purposes",
    "tags": ["test", "dummy", "fake", "sample", "generated", "nonsense", "placeholder"],
    "config": {
      "enableFeatureA": true,
      "enableFeatureB": false,
      "maxRetries": 5,
      "timeout": 30000,
      "batchSize": 100,
      "compression": "gzip",
      "encryption": "aes256"
    }
  },
  "randomNumbers": [
    847.263, 193.847, 572.194, 836.472, 295.738, 649.183, 472.856, 738.294,
    164.729, 583.947, 729.384, 456.728, 384.657, 928.374, 657.293, 837.465,
    293.847, 748.293, 584.729, 374.856, 492.738, 638.472, 847.293, 574.638
  ],
  "gibberish": {
    "words": ["flibber", "jabberwocky", "snurfle", "blunderbuss", "whimsical", "brouhaha"],
    "phrases": [
      "the quick brown fox jumps over the lazy dog but then decides to take a nap",
      "all your base are belong to us and we're not giving them back anytime soon",
      "to be or not to be that is the question but nobody really knows the answer"
    ],
    "sentences": [
      "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
      "Ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
      "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."
    ]
  },
  "moreComplexData": {
    "users": [
      {
        "id": 1001,
        "username": "user_alpha_bravo_charlie",
        "email": "alpha.bravo@example-domain-that-does-not-exist.com",
        "profile": {
          "firstName": "Alpha",
          "lastName": "Bravo", 
          "bio": "This is a very long biographical description that contains lots of meaningless information about this fake user",
          "preferences": {
            "theme": "dark",
            "language": "en-US",
            "notifications": {
              "email": true,
              "push": false,
              "sms": true
            }
          }
        }
      },
      {
        "id": 1002,
        "username": "delta_echo_foxtrot_golf",
        "email": "delta.echo@another-fake-domain.org",
        "profile": {
          "firstName": "Delta",
          "lastName": "Echo",
          "bio": "Another lengthy description filled with random words and meaningless content to increase the size of this JSON file",
          "preferences": {
            "theme": "light",
            "language": "es-ES",
            "notifications": {
              "email": false,
              "push": true,
              "sms": false
            }
          }
        }
      }
    ],
    "products": [
      {
        "sku": "PROD-12345-ABCDE",
        "name": "Super Duper Widget Deluxe Pro Max Ultra",
        "description": "This is an amazing product that does incredible things and will change your life forever or at least until you get bored with it",
        "price": 299.99,
        "categories": ["widgets", "gadgets", "electronics", "home", "office"],
        "specifications": {
          "weight": "2.5 kg",
          "dimensions": "30cm x 20cm x 15cm",
          "material": "premium plastic composite with metal accents",
          "warranty": "2 years limited warranty with extended coverage options available"
        }
      }
    ]
  }
}
"#;

static DATA_TOML: &str = r#"
longArray = [
    "item001",
    "item002",
    "item003",
    "item004",
    "item005",
    "item006",
    "item007",
    "item008",
    "item009",
    "item010",
    "item011",
    "item012",
    "item013",
    "item014",
    "item015",
    "item016",
    "item017",
    "item018",
    "item019",
    "item020",
    "item021",
    "item022",
    "item023",
    "item024",
    "item025",
    "item026",
    "item027",
    "item028",
    "item029",
    "item030",
    "item031",
    "item032",
]
randomNumbers = [
    847.263,
    193.847,
    572.194,
    836.472,
    295.738,
    649.183,
    472.856,
    738.294,
    164.729,
    583.947,
    729.384,
    456.728,
    384.657,
    928.374,
    657.293,
    837.465,
    293.847,
    748.293,
    584.729,
    374.856,
    492.738,
    638.472,
    847.293,
    574.638,
]
randomStrings = [
    "xkjf94jf2kl3j4lk5j6lk7j8lk9j0lk1j2lk3j4lk5j6lk7j8lk9j0lk1j2lk3j4lk5j6lk7j8lk9j0",
    "mqwerty789456123qwertyuiopasdfghjklzxcvbnm098765432109876543210987654321",
    "aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzzaabbccddeeffgghhii",
    "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt",
    "abcdefghijklmnopqrstuvwxyz0123456789abcdefghijklmnopqrstuvwxyz0123456789",
    "fizzbuzzfoobarbazbingbangboomhelloworldgoodbyemoonhellostarsnicetomeetyou",
    "qwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjkl",
    "1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t1u2v3w4x5y6z7a8b9c0d1e2f3g4h",
]

[gibberish]
phrases = [
    "the quick brown fox jumps over the lazy dog but then decides to take a nap",
    "all your base are belong to us and we're not giving them back anytime soon",
    "to be or not to be that is the question but nobody really knows the answer",
]
sentences = [
    "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
    "Ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
    "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
]
words = [
    "flibber",
    "jabberwocky",
    "snurfle",
    "blunderbuss",
    "whimsical",
    "brouhaha",
]

[metadata]
author = "DataGenerator3000"
created = "2024-01-01T00:00:00Z"
description = "This is a large blob of nonsense JSON data created for testing purposes"
modified = "2024-12-31T23:59:59Z"
tags = [
    "test",
    "dummy",
    "fake",
    "sample",
    "generated",
    "nonsense",
    "placeholder",
]
version = "1.0.0"

[metadata.config]
batchSize = 100
compression = "gzip"
enableFeatureA = true
enableFeatureB = false
encryption = "aes256"
maxRetries = 5
timeout = 30000

[[moreComplexData.products]]
categories = [
    "widgets",
    "gadgets",
    "electronics",
    "home",
    "office",
]
description = "This is an amazing product that does incredible things and will change your life forever or at least until you get bored with it"
name = "Super Duper Widget Deluxe Pro Max Ultra"
price = 299.99
sku = "PROD-12345-ABCDE"

[moreComplexData.products.specifications]
dimensions = "30cm x 20cm x 15cm"
material = "premium plastic composite with metal accents"
warranty = "2 years limited warranty with extended coverage options available"
weight = "2.5 kg"

[[moreComplexData.users]]
email = "alpha.bravo@example-domain-that-does-not-exist.com"
id = 1001
username = "user_alpha_bravo_charlie"

[moreComplexData.users.profile]
bio = "This is a very long biographical description that contains lots of meaningless information about this fake user"
firstName = "Alpha"
lastName = "Bravo"

[moreComplexData.users.profile.preferences]
language = "en-US"
theme = "dark"

[moreComplexData.users.profile.preferences.notifications]
email = true
push = false
sms = true

[[moreComplexData.users]]
email = "delta.echo@another-fake-domain.org"
id = 1002
username = "delta_echo_foxtrot_golf"

[moreComplexData.users.profile]
bio = "Another lengthy description filled with random words and meaningless content to increase the size of this JSON file"
firstName = "Delta"
lastName = "Echo"

[moreComplexData.users.profile.preferences]
language = "es-ES"
theme = "light"

[moreComplexData.users.profile.preferences.notifications]
email = false
push = true
sms = false

[nestedObjects.anotherBranch]
moreData = "this is just filler text to make the json larger and more complex"

[[nestedObjects.anotherBranch.arrayOfObjects]]
active = true
id = "abc123"
name = "widget"
value = 99.99

[[nestedObjects.anotherBranch.arrayOfObjects]]
active = false
id = "def456"
name = "gadget"
value = 149.99

[[nestedObjects.anotherBranch.arrayOfObjects]]
active = true
id = "ghi789"
name = "doodad"
value = 24.99

[[nestedObjects.anotherBranch.arrayOfObjects]]
active = false
id = "jkl012"
name = "thingamajig"
value = 199.99

[nestedObjects.level1.level2.level3.level4]
booleans = [
    true,
    false,
    true,
    true,
    false,
    false,
    true,
    false,
    true,
    false,
]
data = "very deep nested data with long string values and meaningless content"
numbers = [
    42,
    3.14159,
    2.71828,
    1.41421,
    1.61803,
    0.57721,
    2.5029,
    4.6692,
]
"#;
