Feature: `on_state_change` callback

  Scenario: Member joined and on_state_change() triggered
    Given room with joined members Alice and Bob
    When Alice receives connection with Bob
    Then Alice gets connected with Bob
    When Bob receives connection with Alice
    Then Bob gets connected with Alice

  Scenario: Member disconnected and on_state_change() triggered
    Given room with joined members Alice and Bob
    When Alice loses WS connection
    Then Alice gets disconnected with Bob

  Scenario: Member reconnected and on_state_change() triggered
    Given room with joined members Alice and Bob
    When Alice loses WS connection
    Then Alice gets disconnected with Bob
    When Alice restores WS connection
    Then Bob gets connected with Alice