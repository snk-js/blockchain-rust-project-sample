import type { Principal } from '@dfinity/principal';
export interface Post {
  'id' : bigint,
  'upvotes' : bigint,
  'body' : string,
  'owner_id' : Principal,
}
export interface PostQuery {
  'page_size' : bigint,
  'page' : bigint,
  'sort' : PostSort,
}
export type PostSort = { 'New' : null } |
  { 'Top' : null };
export interface User { 'id' : Principal, 'username' : string }
export interface _SERVICE {
  'create_post' : (arg_0: string) => Promise<Post>,
  'create_user' : (arg_0: string) => Promise<User>,
  'greet' : (arg_0: string) => Promise<string>,
  'list_users' : () => Promise<Array<User>>,
  'upvote_post' : (arg_0: bigint) => Promise<[] | [Post]>,
}
