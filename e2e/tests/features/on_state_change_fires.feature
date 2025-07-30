Feature: `on_state_change` callback

  Scenario: Member joined and on_state_change() triggered
    Given room with joined member Alice
    And member Bob
    When Bob joins the room
    And Alice receives connection with Bob
    Then Alice gets connected with Bob
    When Bob receives connection with Alice
    Then Bob gets connected with Alice