export const idlFactory = ({ IDL }) => {
  const Box = IDL.Rec();
  const User = IDL.Record({
    'id' : IDL.Text,
    'primitive_two_tuple' : IDL.Tuple(IDL.Text, IDL.Nat64),
  });
  Box.fill(
    IDL.Variant({
      'Bad' : IDL.Tuple(
        IDL.Tuple(IDL.Text, IDL.Nat64),
        IDL.Record({
          'id' : IDL.Text,
          'primitive_two_tuple' : IDL.Tuple(IDL.Text, IDL.Nat64),
        }),
        Box,
      ),
      'Good' : IDL.Null,
    })
  );
  const Reaction = IDL.Variant({
    'Bad' : IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User, Box),
    'Good' : IDL.Null,
  });
  const HttpResponse = IDL.Record({
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const StreamingCallbackType = IDL.Variant({
    'without_headers' : IDL.Null,
    'with_headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  return IDL.Service({
    'complex_one_tuple_inline_param' : IDL.Func(
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64))],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64))],
        ['query'],
      ),
    'complex_one_tuple_inline_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64))],
        ['query'],
      ),
    'complex_one_tuple_param' : IDL.Func(
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64))],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64))],
        ['query'],
      ),
    'complex_one_tuple_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64))],
        ['query'],
      ),
    'complex_three_tuple_inline_param' : IDL.Func(
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User, Reaction)],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User, Reaction)],
        ['query'],
      ),
    'complex_three_tuple_inline_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User, Reaction)],
        ['query'],
      ),
    'complex_three_tuple_param' : IDL.Func(
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User, Box)],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User, Box)],
        ['query'],
      ),
    'complex_three_tuple_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User, Box)],
        ['query'],
      ),
    'complex_two_tuple_inline_param' : IDL.Func(
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User)],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User)],
        ['query'],
      ),
    'complex_two_tuple_inline_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User)],
        ['query'],
      ),
    'complex_two_tuple_param' : IDL.Func(
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User)],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User)],
        ['query'],
      ),
    'complex_two_tuple_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Tuple(IDL.Text, IDL.Nat64), User)],
        ['query'],
      ),
    'primitive_one_tuple_inline_param' : IDL.Func(
        [IDL.Tuple(IDL.Text)],
        [IDL.Tuple(IDL.Text)],
        ['query'],
      ),
    'primitive_one_tuple_inline_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Text)],
        ['query'],
      ),
    'primitive_one_tuple_param' : IDL.Func(
        [IDL.Tuple(IDL.Text)],
        [IDL.Tuple(IDL.Text)],
        ['query'],
      ),
    'primitive_one_tuple_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Text)],
        ['query'],
      ),
    'primitive_three_tuple_inline_param' : IDL.Func(
        [IDL.Tuple(IDL.Text, IDL.Nat64, IDL.Principal)],
        [IDL.Tuple(IDL.Text, IDL.Nat64, IDL.Principal)],
        ['query'],
      ),
    'primitive_three_tuple_inline_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Text, IDL.Nat64, IDL.Principal)],
        ['query'],
      ),
    'primitive_three_tuple_param' : IDL.Func(
        [IDL.Tuple(IDL.Text, IDL.Nat64, IDL.Principal)],
        [IDL.Tuple(IDL.Text, IDL.Nat64, IDL.Principal)],
        ['query'],
      ),
    'primitive_three_tuple_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Text, IDL.Nat64, IDL.Principal)],
        ['query'],
      ),
    'primitive_two_tuple_inline_param' : IDL.Func(
        [IDL.Tuple(IDL.Text, IDL.Text)],
        [IDL.Tuple(IDL.Text, IDL.Text)],
        ['query'],
      ),
    'primitive_two_tuple_inline_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Text, IDL.Text)],
        ['query'],
      ),
    'primitive_two_tuple_param' : IDL.Func(
        [IDL.Tuple(IDL.Text, IDL.Nat64)],
        [IDL.Tuple(IDL.Text, IDL.Nat64)],
        ['query'],
      ),
    'primitive_two_tuple_return_type' : IDL.Func(
        [],
        [IDL.Tuple(IDL.Text, IDL.Nat64)],
        ['query'],
      ),
    'tuple_array_params_and_return_type' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
        ['query'],
      ),
    'tuple_array_record_field' : IDL.Func([], [HttpResponse], ['query']),
    'tuple_array_variant_field' : IDL.Func(
        [],
        [StreamingCallbackType],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };