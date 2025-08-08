Feature: `on_state_change` callback

  Scenario Outline: Member joined and on_state_change() triggered
    Given room with joined members Alice and Bob
    When Alice receives connection with Bob
    And Bob receives connection with Alice
    Then Alice's connection with Bob is <connection_state>
    And Bob's connection with Alice is <connection_state>

    @mesh
    Examples:
      | connection_state  |
      | P2P::Failed    |
    # TOOD: Implement for SFU.
    @sfu
    Examples:
      | connection_state  |
      | None              |

  Scenario Outline: Member disconnected and on_state_change() triggered
    Given room with joined members Alice and Bob
    When Alice loses WS connection
    Then Alice's connection with Bob is <connection_state>

    @mesh
    Examples:
      | connection_state  |
      | P2P::Connected |
    # TOOD: Implement for SFU.
    @sfu
    Examples:
      | connection_state  |
      | None              |