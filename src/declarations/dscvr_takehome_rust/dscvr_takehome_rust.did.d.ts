import type { Principal } from '@dfinity/principal';
export interface Post {
  'id' : bigint,
  'upvotes' : bigint,
  'voted_by' : Array<[] | [string]>,
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
  'list_posts' : (arg_0: PostQuery) => Promise<Array<Post>>,
  'list_posts_by_user' : (arg_0: Principal) => Promise<Array<Post>>,
  'list_users' : () => Promise<Array<User>>,
  'upvote_post' : (arg_0: bigint) => Promise<[] | [Post]>,
}
