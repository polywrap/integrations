License: Apache-2.0


# A forum app pallet
- Using  4 storage
    - AllPost
        - All user post is stored in this storage map
    - AllComments
        - All user comments is stored in this storage map
    - Kids
        - Describes the heirarchy of where a comment is replying to, it could be a reply to a post or another comment
    - ItemCounter
        - Serves as our unique ID which is incremented each time a post or a comment is submitted
