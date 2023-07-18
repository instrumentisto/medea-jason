Feature: Room closing

  @both
  Scenario: `Room.on_close()` fires when `Jason.close_room()` is invoked
    Given room with joined member Alice
    When Alice's room closed by client
    Then Alice's `on_close` room's callback fires with `RoomClosed` reason

  @both
  Scenario: `Room.on_close()` fires when `Jason.dispose()` is invoked
    Given room with joined member Alice
    When Alice disposes Jason object
    Then Alice's `on_close` room's callback fires with `RoomClosed` reason

  @both
  Scenario: `Room.on_close()` fires when member is removed by Control API
    Given room with joined member Alice
    When Control API removes member Alice
    Then Alice's `on_close` room's callback fires with `Evicted` reason

  @both
  Scenario: `Room.on_close()` fires when room is removed by Control API
    Given room with joined member Alice
    When Control API removes the room
    Then Alice's `on_close` room's callback fires with `Evicted` reason
