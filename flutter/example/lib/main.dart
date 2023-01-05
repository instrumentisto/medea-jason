import 'package:flutter/material.dart';
import 'join_route.dart';

void main() async {
  runApp(MaterialApp(
    title: 'Medea demo',
    initialRoute: '/',
    routes: {
      '/': (context) => JoinRoute(),
    },
  ));
}
