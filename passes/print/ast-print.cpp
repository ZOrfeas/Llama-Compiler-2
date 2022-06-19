
#include <string>

#include "../../typesystem/utils.hpp"
#include "./ast-print.hpp"
#include "../../ast/ast.hpp"

const std::string PrintVisitor::full_vert = "│";
const std::string PrintVisitor::split_vert = "├─";
const std::string PrintVisitor::half_vert =  "└─";

// Constructor

PrintVisitor::PrintVisitor(std::ostream& out): out(out) {}

// Utilities

PrintVisitor::depth_guard::depth_guard(PrintVisitor &v): v(v) {
    v.incr_depth();
}
PrintVisitor::depth_guard::~depth_guard() {
    v.decr_depth();
}

void PrintVisitor::incr_depth() {
    depth++;
    stack.push_back(true);
}
void PrintVisitor::decr_depth() {
    depth--;
    stack.pop_back();
}

std::string PrintVisitor::gen_prefix() {
    std::string prefix = "";
    if (depth == 0) return "";
    for (int i = 0; i < depth-1; i++) {
        if (stack[i]) prefix += " " + full_vert;
        else prefix += "  ";
        // prefix += " " + full_vert;
    }
    if (is_last) {
        prefix += " " + half_vert;
        is_last = false;
        stack.back() = false;
    }
    else prefix += " " + split_vert;
    return prefix;
}

void PrintVisitor::println_with_prefix(std::string_view s) {
    out << gen_prefix() << s << '\n';
}

// Visit method overrides

void PrintVisitor::visit(ast::core::Program *program) {
    const auto stmt_cnt = program->defstmt_list->size();
    println_with_prefix("Program (" + std::to_string(stmt_cnt) + " statements)");
    auto guard = depth_guard(*this);
    for (auto const& stmt : *program->defstmt_list) {
        if (&stmt == &program->defstmt_list->back())
            is_last = true;
        stmt->accept(this);
    }
    is_last = false;
}
void PrintVisitor::visit(ast::stmt::TypeStmt *type_stmt) {
    const auto type_cnt = type_stmt->def_list->size();
    println_with_prefix("TypeStmt (" + std::to_string(type_cnt) + " typedefs)");
    auto guard = depth_guard(*this);
    for (auto const& type_def : *type_stmt->def_list) {
        if (&type_def == &type_stmt->def_list->back()) 
            is_last = true;
        type_def->accept(this);
    }
    is_last = false;
}
void PrintVisitor::visit(ast::def::TypeDef *type_def) {
    const auto constr_cnt = type_def->constructor_list->size();
    println_with_prefix("TypeDef (" + std::to_string(constr_cnt) + " constructors)");
    auto guard = depth_guard(*this);
    for (auto const& constr : *type_def->constructor_list) {
        if (&constr == &type_def->constructor_list->back())
            is_last = true;
        constr->accept(this);
    }
    is_last = false;
}
void PrintVisitor::visit(ast::stmt::LetStmt *let_stmt) {
    const auto def_cnt = let_stmt->def_list->size();
    const std::string rec_string = let_stmt->is_recursive ? "recursive" : "non-recursive";
    println_with_prefix("LetStmt (" + rec_string + " " + std::to_string(def_cnt) + " definitions)");
    auto guard = depth_guard(*this);
    for (auto const& def : *let_stmt->def_list) {
        if (&def == &let_stmt->def_list->back())
            is_last = true;
        def->accept(this);
    }
    is_last = false;
}
void PrintVisitor::visit(ast::def::Constant *cnst) {
    println_with_prefix("Constant (" + cnst->id + ")");
    auto guard = depth_guard(*this);
    if (cnst->type_annotation != nullptr) {
        cnst->type_annotation->accept(this);
    }
    is_last = true;
    cnst->expr->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::def::Function *fn) {
    println_with_prefix("Function (" + fn->id + ")");
    auto guard = depth_guard(*this);
    if (fn->type_annotation != nullptr) {
        fn->type_annotation->accept(this);
    }
    for (auto const& param : *fn->param_list) {
        param->accept(this);
    }
    is_last = true;
    fn->expr->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::def::Array *arr) {
    println_with_prefix("Array (" + arr->id + ")");
    auto guard = depth_guard(*this);
    if (arr->type_annotation != nullptr) {
        arr->type_annotation->accept(this);
    }

    for (auto const& dim : *arr->dim_expr_list) {
        if (&dim == &arr->dim_expr_list->back())
            is_last = true;
        dim->accept(this);
    }
    is_last = false;
}
void PrintVisitor::visit(ast::def::Variable *var) {
    println_with_prefix("Variable (" + var->id + ")");
    auto guard = depth_guard(*this);
    is_last = true;
    if (var->type_annotation != nullptr) {
        var->type_annotation->accept(this);
    }
    is_last = false;
}

void PrintVisitor::visit(ast::expr::LetIn *let_in) {
    println_with_prefix("LetIn");
    auto guard = depth_guard(*this);
    let_in->def->accept(this);
    is_last = true;
    let_in->expr->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::expr::literal::Unit *unit) {
    println_with_prefix("Unit literal");
}
void PrintVisitor::visit(ast::expr::literal::Bool *boolean) {
    const std::string bool_string = boolean->val ? "true" : "false";
    println_with_prefix("Bool literal (" + bool_string + ")");
}
void PrintVisitor::visit(ast::expr::literal::Int *integer) {
    println_with_prefix("Int literal (" + integer->original_val + ")");
}
void PrintVisitor::visit(ast::expr::literal::Char *chr) {
    println_with_prefix("Char literal (" + chr->original_val + ")");
}
void PrintVisitor::visit(ast::expr::literal::Float *flt) {
    println_with_prefix("Float literal (" + flt->original_val + ")");
}
void PrintVisitor::visit(ast::expr::literal::String *str) {
    println_with_prefix("String literal (" + str->original_val + ")");
}
void PrintVisitor::visit(ast::expr::op::Binary *binop) {
    println_with_prefix(std::string("Binary operator (") + (char)binop->op + ")");
    auto guard = depth_guard(*this);
    binop->lhs->accept(this);
    is_last = true;
    binop->rhs->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::expr::op::Unary *unop) {
    println_with_prefix(std::string("Unary operator (") + (char)unop->op + ")");
    auto guard = depth_guard(*this);
    is_last = true;
    unop->expr->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::expr::op::New *newop) {
    println_with_prefix("New");
    auto guard = depth_guard(*this);
    is_last = true;
    newop->t->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::expr::While *while_expr) {
    println_with_prefix("While");
    auto guard = depth_guard(*this);
    while_expr->cond->accept(this);
    is_last = true;
    while_expr->body->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::expr::For *for_expr) {
    println_with_prefix("For");
    auto guard = depth_guard(*this);
    println_with_prefix("Id " + for_expr->id);
    for_expr->init->accept(this);
    println_with_prefix(for_expr->ascending ? "to" : "downto");
    for_expr->end->accept(this);
    is_last = true;
    for_expr->body->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::expr::If *if_expr) {
    println_with_prefix("If");
    auto guard = depth_guard(*this);
    if_expr->cond->accept(this);
    is_last = true;
    if (if_expr->else_expr != nullptr) is_last = false;
    if_expr->then_expr->accept(this);
    if (if_expr->else_expr != nullptr) {
        is_last = true;
        if_expr->else_expr->accept(this);
    }
    is_last = false;
}
void PrintVisitor::visit(ast::expr::Dim *dim_expr) {
    println_with_prefix("Dim (on id" + dim_expr->id + ")");
    auto guard = depth_guard(*this);
    is_last = true;
    dim_expr->dim->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::expr::IdCall *id_call) {
    println_with_prefix("IdCall (" + id_call->id + ")");
}
void PrintVisitor::visit(ast::expr::FuncCall *fn_call) {
    println_with_prefix("FuncCall (" + fn_call->id + ")");
    auto guard = depth_guard(*this);
    for (auto const& arg : *fn_call->arg_list) {
        if (&arg == &fn_call->arg_list->back())
            is_last = true;
        arg->accept(this);
    }
    is_last = false;
}
void PrintVisitor::visit(ast::expr::ConstrCall *cnstr_call) {
    println_with_prefix("ConstrCall (" + cnstr_call->id + ")");
    auto guard = depth_guard(*this);
    for (auto const& arg : *cnstr_call->arg_list) {
        if (&arg == &cnstr_call->arg_list->back())
            is_last = true;
        arg->accept(this);
    }
    is_last = false;
}
void PrintVisitor::visit(ast::expr::ArrayAccess *arr_access) {
    println_with_prefix("ArrayAccess (" + arr_access->id + ")");
    auto guard = depth_guard(*this);
    for (auto const& idx : *arr_access->index_list) {
        if (&idx == &arr_access->index_list->back())
            is_last = true;
        idx->accept(this);
    }
    is_last = false;
}
void PrintVisitor::visit(ast::expr::Match *match_expr) {
    println_with_prefix("Match");
    auto guard = depth_guard(*this);
    match_expr->to_match->accept(this);
    for (auto const& clause : *match_expr->clause_list) {
        if (&clause == &match_expr->clause_list->back())
            is_last = true;
        clause->accept(this);
    }
    is_last = false;
}

void PrintVisitor::visit(ast::annotation::BasicType *basic_type) {
    println_with_prefix(std::string("BasicType (") + typesys::type_enum_to_str(basic_type->t) + ")");
}
void PrintVisitor::visit(ast::annotation::FunctionType *fn_type) {
    println_with_prefix("FunctionType");
    auto guard = depth_guard(*this);
    fn_type->lhs->accept(this);
    is_last = true;
    fn_type->rhs->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::annotation::ArrayType *arr_type) {
    println_with_prefix("ArrayType (" + std::to_string(arr_type->dims) + ")");
    auto guard = depth_guard(*this);
    is_last = true;
    arr_type->contained_type->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::annotation::RefType *ref_type) {
    println_with_prefix("RefType");
    auto guard = depth_guard(*this);
    is_last = true;
    ref_type->contained_type->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::annotation::CustomType *custom_type) {
    println_with_prefix("CustomType (" + custom_type->id + ")");
}

void PrintVisitor::visit(ast::utils::def::Constructor *constructor) {
    println_with_prefix("Constructor (" + constructor->id + ")");
    auto guard = depth_guard(*this);
    for (auto const& type : *constructor->type_list) {
        if (&type == &constructor->type_list->back())
            is_last = true;
        type->accept(this);
    }
    is_last = false;
}
void PrintVisitor::visit(ast::utils::def::Param *param) {
    println_with_prefix("Param (" + param->id + ")");
    if (param->type_annotation != nullptr) {
        auto guard = depth_guard(*this);
        is_last = true;
        param->type_annotation->accept(this);
        is_last = false;
    }
}

void PrintVisitor::visit(ast::utils::match::PatLiteral *pat_literal) {
    println_with_prefix("PatLiteral");
    auto guard = depth_guard(*this);
    is_last = true;
    pat_literal->literal->accept(this);
    is_last = false;
}
void PrintVisitor::visit(ast::utils::match::PatId *pat_id) {
    println_with_prefix("PatId (" + pat_id->id + ")");
}
void PrintVisitor::visit(ast::utils::match::PatConstr *pat_constr) {
    println_with_prefix("PatConstr (" + pat_constr->id + ")");
    auto guard = depth_guard(*this);
    for (auto const& arg : *pat_constr->pattern_list) {
        if (&arg == &pat_constr->pattern_list->back())
            is_last = true;
        arg->accept(this);
    }
    is_last = false;
}
void PrintVisitor::visit(ast::utils::match::Clause *clause) {
    println_with_prefix("Clause");
    auto guard = depth_guard(*this);
    clause->pattern->accept(this);
    is_last = true;
    clause->expr->accept(this);
    is_last = false;
}