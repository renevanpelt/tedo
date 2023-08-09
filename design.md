## Design





The toml:


```toml

[[lists]]
    name = "Links"
    [[lists.fields]]
    name        = "title"
    label       = "Title"

    [[lists.fields]]
    name        = "url"
    label       = "Url"
    

[[extension_actions]]
    name            = 'Add Page to "Links"'
    type            = 'button'
    action          = 'add_to_list'
    [[extensions_actions.mappings]]
    selector    = 'title'
    field       = 'title'
    [[extensions_actions.mappings]]
    selector    = 'title'
    field       = 'title'

    

```