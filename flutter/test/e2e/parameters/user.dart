// ignore_for_file: constant_identifier_names

import 'package:gherkin/gherkin.dart';

/// [User]s available in the [UsersParameter].
enum TestUser {
  Alice,
  Bob,
  Carol,
}

/// [CustomParameter] of [TestUser]s representing a [User] of a test.
class UsersParameter extends CustomParameter<TestUser> {
  UsersParameter()
      : super(
          'user',
          RegExp(
            '(${TestUser.values.map((e) => e.name).join('|')})',
            caseSensitive: true,
          ),
          (c) => TestUser.values.firstWhere((e) => e.name == c),
        );
}
