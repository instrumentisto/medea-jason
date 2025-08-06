Feature: `on_state_change` callback

  Scenario Outline: Member joined and on_state_change() triggered
    Given room with joined members Alice and Bob
    When Alice receives connection with Bob
    Then Alice's connection with Bob is <connection_state>
    When Bob receives connection with Alice
    Then Bob's connection with Alice is <connection_state>

    @mesh
    Examples:
      | connection_state  |
      | P2P::Connected    |

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
      | P2P::Disconnected |

    @sfu
    Examples:
      | connection_state  |
      | None              |