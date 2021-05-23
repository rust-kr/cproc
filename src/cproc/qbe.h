#ifndef __QBE_H__
#define __QBE_H__

#include <stdbool.h>
#include <inttypes.h>
#include "util.h"
#include "cc.h"

struct name {
	char *str;
	uint64_t id;
};

struct repr {
	char base;
	char ext;
};

struct value {
	enum {
		VALUE_NONE,
		VALUE_GLOBAL,
		VALUE_CONST,
		VALUE_TEMP,
		VALUE_TYPE,
	} kind;
	struct repr *repr;
	union {
		struct name name;
		uint64_t i;
		double f;
	};
};

struct lvalue {
	struct value *addr;
	struct bitfield bits;
};

enum instkind {
	INONE,

#define OP(op, name) op,
#include "ops.h"
#undef OP

	IARG,
};

struct inst {
	enum instkind kind;
	struct value res, *arg[2];
};

struct jump {
	enum {
		JUMP_NONE,
		JUMP_JMP,
		JUMP_JNZ,
		JUMP_RET,
	} kind;
	struct value *arg;
	struct block *blk[2];
};

struct block {
	struct name label;
	struct array insts;
	struct {
		struct block *blk[2];
		struct value *val[2];
		struct value res;
	} phi;
	struct jump jump;

	struct block *next;
};

struct switchcase {
	struct treenode node;
	struct block *body;
};

struct func {
	struct decl *decl, *namedecl;
	char *name;
	struct type *type;
	struct block *start, *end;
	struct map *gotos;
	uint64_t lastid;
};

struct repr i8 = {'w', 'b'};
struct repr i16 = {'w', 'h'};
struct repr i32 = {'w', 'w'};
struct repr i64 = {'l', 'l'};
struct repr f32 = {'s', 's'};
struct repr f64 = {'d', 'd'};
struct repr iptr = {'l', 'l'};


#endif