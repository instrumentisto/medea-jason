import 'package:flutter/material.dart';
import 'join_route.dart';

void main() {
  runApp(MaterialApp(
    title: 'Medea demo',
    initialRoute: '/',
    routes: {
      '/': (context) => const JoinRoute(),
    },
  ));
}
