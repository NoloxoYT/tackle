# Tackle

## How to use it?

simply clone and build it

## What OS ?

for now the packages are Arch only, but you can port them to any OS by
following [these steps](#making-your-own-modules)

## Packages

### Current Packages

| Module      | OS   | Since[^1] |
| ----------- | ---- | --------- |
| containers  | Arch | V1        |
| desktop     | Arch | V1        |
| development | Arch | V1        |
| editors     | Arch | V1        |
| essentials  | Arch | V1        |
| fonts       | Arch | V1        |
| manifest    | Arch | V1        |
| media       | Arch | V1        |
| networking  | Arch | V1        |
| pipewire    | Arch | V1        |

[^1]: Manifest version

<a id="making-your-own-modules"></a>

### Making your own modules

#### Types

The types that are currently supported are:

- Bool
- Choices

#### Example

let's create a simple module to print 'hello world!'

it will look like this:

```json
{
  "name": "Hello World Module",
  "parameters": [
    {
      "param_name": "Print word",
      "param_type": {
        "Choice": {
          "choices": ["Hello", "World!"],
          "command": [
            {
              "base": "echo 'Hello'",
              "need_additional_input": false,
              "additional_input_description": "",
              "show_output": true
            },
            {
              "base": "echo 'World!'",
              "need_additional_input": false,
              "additional_input_description": "",
              "show_output": true
            }
          ]
        }
      }
    },
    {
      "param_name": "Print whole",
      "param_type": {
        "Bool": {
          "command": {
            "base": "echo 'Hello World!'",
            "need_additional_input": false,
            "additional_input_description": "",
            "show_output": true
          }
        }
      }
    }
  ]
}
```

#### Contributing

To contribute (by adding modules), simply create an module, add it to the manifest
and make a PR or fork it and use your own version

[LICENSE]: ./LICENSE
