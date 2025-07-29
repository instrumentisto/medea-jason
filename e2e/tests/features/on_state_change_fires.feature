Feature: `on_state_change` callback

  Scenario: Member joined
    Given room with joined member Alice
    And member Bob
    When Bob joins the room
    When Alice receives connection with Bob
    Then Alice gets connected with Bob
    And Bob gets connected with Alice
