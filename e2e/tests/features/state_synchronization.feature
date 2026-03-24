Feature: State synchronization

  Scenario: Remote track disable works while disconnect
    Given room with joined member Alice and Bob
    When Alice loses WS connection
    And Bob disables audio and awaits it completes
    And Alice restores WS connection
    Then Alice's audio remote track from Bob is disabled

