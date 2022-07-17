#include <algorithm>
#include <array>
#include <memory>
#include <spdlog/spdlog.h>

#include "./sem.hpp"
#include "../../ast/ast.hpp"

// Constructor

SemVisitor::SemVisitor() {}

// Utilities



// Visit method overloads

void SemVisitor::visit(ast::core::Program* program) {
    spdlog::trace("[sem::SemVisitor] running sem on program");
    for (auto& def : *program->defstmt_list) {
        def->accept(this);
    }
}
void SemVisitor::visit(ast::stmt::TypeStmt* type_stmt) {
    spdlog::trace("[sem::SemVisitor] running sem on type_stmt");
    auto& def_list = *type_stmt->def_list;
    std::vector<typesys::Type> types(def_list.size());
    for (size_t i = 0; i < def_list.size(); i++) {
        auto& tdef = def_list[i];
        types[i] = typesys::Type::get<typesys::Custom>(tdef->id);
        if (!tt.insert(tdef->id, tdef.get(), types[i]))
            error::crash<error::SEMANTIC>("type '{}' redeclared", tdef->id);
    }
    for (size_t i = 0; i < def_list.size(); i++) {
        auto& tdef = def_list[i];
        passed_type = types[i];
        tdef->accept(this);
    }
}
void SemVisitor::visit(ast::def::TypeDef* type_def) {
    spdlog::trace("[sem::SemVisitor] running sem on type_def {}", type_def->id);
    auto my_type = passed_type;
    for (auto& constructor : *type_def->constructor_list) {
        constructor->accept(this);
        my_type->unsafe_as<typesys::Custom>()
            ->constructor_types.push_back(
                passed_type->unsafe_as<typesys::Constructor>()
            );
    }
    passed_type = my_type;
}
void SemVisitor::visit(ast::stmt::LetStmt* let_stmt) {
    auto insert_let_names = [](ast::stmt::LetStmt* let_stmt) {
        for (auto& def : *let_stmt->def_list) {
            st.insert(def->id, def, )
        }
    }
    if (let_stmt->is_recursive) {

    }
    for (auto& def : *let_stmt->def_list) {

    }
    if (!let_stmt->is_recursive) {

    }

}
void SemVisitor::visit(ast::def::Constant* cnst) {}
void SemVisitor::visit(ast::def::Function* fn) {}
void SemVisitor::visit(ast::def::Array* arr) {}
void SemVisitor::visit(ast::def::Variable* var) {}

void SemVisitor::visit(ast::expr::LetIn* let_in) {}
void SemVisitor::visit(ast::expr::literal::Unit* unit) {}
void SemVisitor::visit(ast::expr::literal::Bool* boolean) {}
void SemVisitor::visit(ast::expr::literal::Int* integer) {}
void SemVisitor::visit(ast::expr::literal::Char* chr) {}
void SemVisitor::visit(ast::expr::literal::Float* flt) {}
void SemVisitor::visit(ast::expr::literal::String* str) {}
void SemVisitor::visit(ast::expr::op::Binary* binop) {}
void SemVisitor::visit(ast::expr::op::Unary* unop) {}
void SemVisitor::visit(ast::expr::op::New* newop) {}
void SemVisitor::visit(ast::expr::While* while_expr) {}
void SemVisitor::visit(ast::expr::For* for_expr) {}
void SemVisitor::visit(ast::expr::If* if_expr) {}
void SemVisitor::visit(ast::expr::Dim* dim_expr) {}
void SemVisitor::visit(ast::expr::IdCall* id_call) {}
void SemVisitor::visit(ast::expr::FuncCall* fn_call) {}
void SemVisitor::visit(ast::expr::ConstrCall* cnstr_call) {}
void SemVisitor::visit(ast::expr::ArrayAccess* arr_access) {}
void SemVisitor::visit(ast::expr::Match* match_expr) {}

void SemVisitor::visit(ast::annotation::BasicType* basic_type) {}
void SemVisitor::visit(ast::annotation::FunctionType* fn_type) {}
void SemVisitor::visit(ast::annotation::ArrayType* arr_type) {}
void SemVisitor::visit(ast::annotation::RefType* ref_type) {}
void SemVisitor::visit(ast::annotation::CustomType* custom_type) {}

void SemVisitor::visit(ast::utils::def::Constructor* constructor) {}
void SemVisitor::visit(ast::utils::def::Param* param) {}

void SemVisitor::visit(ast::utils::match::PatLiteral* pat_literal) {}
void SemVisitor::visit(ast::utils::match::PatId* pat_id) {}
void SemVisitor::visit(ast::utils::match::PatConstr* pat_constr) {}
void SemVisitor::visit(ast::utils::match::Clause* clause) {}
