export const idlFactory = ({ IDL }) => {
  const Post = IDL.Record({
    'id' : IDL.Nat64,
    'upvotes' : IDL.Nat64,
    'body' : IDL.Text,
    'owner_id' : IDL.Principal,
  });
  const User = IDL.Record({ 'id' : IDL.Principal, 'username' : IDL.Text });
  return IDL.Service({
    'create_post' : IDL.Func([IDL.Text], [Post], []),
    'create_user' : IDL.Func([IDL.Text], [User], []),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'list_users' : IDL.Func([], [IDL.Vec(User)], ['query']),
    'upvote_post' : IDL.Func([IDL.Nat64], [IDL.Opt(Post)], []),
  });
};
export const init = ({ IDL }) => { return []; };
