export const idlFactory = ({ IDL }) => {
  const Post = IDL.Record({
    'id' : IDL.Nat64,
    'upvotes' : IDL.Nat64,
    'voted_by' : IDL.Vec(IDL.Opt(IDL.Text)),
    'body' : IDL.Text,
    'owner_id' : IDL.Principal,
  });
  const User = IDL.Record({ 'id' : IDL.Principal, 'username' : IDL.Text });
  const PostSort = IDL.Variant({ 'New' : IDL.Null, 'Top' : IDL.Null });
  const PostQuery = IDL.Record({
    'page_size' : IDL.Nat64,
    'page' : IDL.Nat64,
    'sort' : PostSort,
  });
  return IDL.Service({
    'create_post' : IDL.Func([IDL.Text], [Post], []),
    'create_user' : IDL.Func([IDL.Text], [User], []),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'list_posts' : IDL.Func([PostQuery], [IDL.Vec(Post)], []),
    'list_posts_by_user' : IDL.Func([IDL.Principal], [IDL.Vec(Post)], []),
    'list_users' : IDL.Func([], [IDL.Vec(User)], ['query']),
    'upvote_post' : IDL.Func([IDL.Nat64], [IDL.Opt(Post)], []),
  });
};
export const init = ({ IDL }) => { return []; };
