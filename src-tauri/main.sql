/*
 Navicat Premium Data Transfer

 Source Server         : tauri-app
 Source Server Type    : SQLite
 Source Server Version : 3035005
 Source Schema         : main

 Target Server Type    : SQLite
 Target Server Version : 3035005
 File Encoding         : 65001

 Date: 28/11/2023 11:21:21
*/

PRAGMA foreign_keys = false;

-- ----------------------------
-- Table structure for auth_header
-- ----------------------------
DROP TABLE IF EXISTS "auth_header";
CREATE TABLE "auth_header" (
  "key" TEXT NOT NULL,
  "Authorization" TEXT NOT NULL,
  "refresh_token" TEXT,
  "remember_me" integer,
  PRIMARY KEY ("key")
);

-- ----------------------------
-- Table structure for tb_user
-- ----------------------------
DROP TABLE IF EXISTS "tb_user";
CREATE TABLE "tb_user" (
  "id" INTEGER NOT NULL,
  "username" TEXT,
  "nickName" TEXT,
  "avatarPath" TEXT,
  "createdBy" integer,
  "createdTime" integer,
  PRIMARY KEY ("id")
);

PRAGMA foreign_keys = true;
